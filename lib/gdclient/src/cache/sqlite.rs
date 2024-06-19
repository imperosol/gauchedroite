use crate::cache::{History, Query, QueryDetailed};
use crate::Orientation;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Connection, OptionalExtension, Row, ToSql};
use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};


/// La version du schéma de la db de cache
const CURRENT_USER_VERSION: i64 = 1;

#[cfg(windows)]
fn cache_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .as_deref()
        .map(|p| Path::new(p).join(".cache/gd"))
}

#[cfg(unix)]
fn cache_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .as_deref()
        .map(|p| Path::new(p).join(".cache/gd"))
}

/// Un cache utilisant une db sqlite.
pub struct SqliteCache {
    conn: Connection,
}

impl SqliteCache {
    /// Initialise la connection à la db de cache.
    ///
    /// Si le fichier `cache.db` n'existe pas encore,
    /// ou bien si son schéma est incorrect,
    /// alors la base de données est initialisée.
    pub fn connect() -> Self {
        let mut path = cache_dir().unwrap();
        if !path.exists() {
            create_dir_all(&path).unwrap();
        }
        path.push("cache.db");

        let conn = Connection::open(path).unwrap();
        let user_version = conn.query_row("pragma user_version", [], |r| r.get(0));
        // Si le pragma n'est pas trouvé ou que la version qu'il donne n'est pa la bonne,
        // on initialise la base de données.
        // Faire une requête de sélection du pragma et une autre d'initialisation
        // est un peu plus long que faire une seule requête d'initialisation
        // avec des "create table if not exists", mais ça permet
        // de faire en sorte que la requête d'initialisation ne tourne
        // qu'à la première exécution du programme, ce qui résulte en un gain de temps.
        if user_version != Ok(CURRENT_USER_VERSION) {
            conn.execute_batch(include_str!("../../sql/initialize_cache.sql"))
                .expect("Couldn't initialize the database");
        }
        Self { conn }
    }
}


// NOTE : c'est pas ouf, comme modélisation.
// Ca serait peut-être mieux, au lieu d'avoir une interface
// qui définit des fonctions qui accomplissent un seul truc
// (ça fait que pour chaque opération, il faut rajouter une nouvelle fonction),
// de définir un builder de requête.
// Comme ça, par exemple, si on cherche la dernière requête
// concernant Macron, on peut écrire : SqliteCache::new().query().input("Macron").last()
// Par contre, ça nécessite de faire un mini-ORM maison,
// donc ça demande un peu de taff

impl History for SqliteCache {
    type Error = rusqlite::Error;

    fn last(&self) -> Result<Option<QueryDetailed>, Self::Error> {
        self.conn
            .query_row(
                r#"select i.content, query.orientation, query.timestamp from query
                inner join input as i on i.id = query.input
                order by query.id desc
                limit 1"#,
                [],
                |row| row.try_into(),
            )
            .optional()
    }

    fn all(&self) -> Result<Vec<QueryDetailed>, Self::Error> {
        Ok(self
            .conn
            .prepare(
                r#"select i.content, query.orientation, query.timestamp from query
                inner join input as i on i.id = query.input;"#,
            )?
            .query_map((), |row| row.try_into())?
            .map(|q| q.unwrap())
            .collect())
    }

    fn last_input(&self, input: &str) -> Result<Option<QueryDetailed>, Self::Error> {
        self.conn
            .prepare(
                r#"select i.content, query.orientation, query.timestamp from query
                inner join input as i on i.id = query.input
                where i.content = ?1
                order by query.id desc
                limit 1;"#,
            )?
            .query_row([input], |row| row.try_into())
            .optional()
    }

    fn push(&mut self, obj: Query) -> Result<(), Self::Error> {
        let transaction = self.conn.transaction()?;
        transaction.execute(
            "insert or ignore into input (content) values (?1)",
            [&obj.input],
        )?;
        // YAKA: si on veut gagner en performance, on pourrait utiliser
        // la fonction "last_insert_rowid", pour ne pas avoir
        // à faire la jointure sur la table input quand il y a eu insert
        transaction.execute(
            r#"insert into query (input, orientation)
            select i.id, 2 from input as i
            where i.content = ?1;"#,
            [&obj.input],
        )?;
        transaction.commit()
    }
}

impl ToSql for Orientation {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let res = match self {
            Orientation::Gauche => 1,
            Orientation::Droite => 2,
            Orientation::GaucheEtDroite => 3,
        };
        Ok(ToSqlOutput::from(res))
    }
}

impl FromSql for Orientation {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_i64().and_then(|i| match i {
            1 => Ok(Orientation::Gauche),
            2 => Ok(Orientation::Droite),
            3 => Ok(Orientation::GaucheEtDroite),
            _ => Err(FromSqlError::OutOfRange(i)),
        })
    }
}

impl<'a> TryFrom<&'a Row<'a>> for QueryDetailed {
    type Error = rusqlite::Error;

    fn try_from(row: &'a Row<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            input: row.get(0)?,
            orientation: row.get(1)?,
            timestamp: row.get(2)?,
        })
    }
}
