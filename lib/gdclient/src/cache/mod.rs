mod sqlite;

use crate::Orientation;
use chrono::{DateTime, Utc};
pub use sqlite::SqliteCache;

pub struct Query {
    pub input: String,
    pub orientation: Orientation,
}

#[derive(Debug)]
pub struct QueryDetailed {
    pub input: String,
    pub orientation: Orientation,
    pub timestamp: DateTime<Utc>,
}

pub trait History {
    type Error;

    /// Retourne les détails de la dernière requête effectuée.
    ///
    /// Si la recherche a échoué, retourne une erreur,
    /// Sinon retourne le résultat.
    /// Même en cas de succès, il peut n'y avoir aucun résultat.
    ///
    /// # Examples
    ///
    /// ```
    /// use gdclient::cache::{History, QueryDetailed};
    ///
    /// fn foo(history: impl History) {
    ///     match history.last() {
    ///         Ok(Some(res)) => println!("Dernière recherche effectuée : {res:?}"),
    ///         Ok(None) => println!("Aucune recherche existante dans le cache"),
    ///         Err(err) => eprintln!("Erreur durant la recherche : {err:?}"),
    ///     };
    /// }
    /// ```
    fn last(&self) -> Result<Option<QueryDetailed>, Self::Error>;

    /// Retourne les détails de toutes les requêtes effectuées.
    ///
    /// En cas de succès, la fonction retourne un Vec au lieu
    /// d'un itérateur, parce que les implémentations peuvent
    /// utiliser dans leur fonctionnement des variables de connexion
    /// qui sont libérées à la sortie de la fonction.
    /// Les restrictions de durée de vie obligent donc
    /// à tout récupérer dans la fonction et donc allouer un Vec.
    fn all(&self) -> Result<Vec<QueryDetailed>, Self::Error>;

    /// Retourne le résultat de la dernière requête sur cette entrée.
    ///
    /// Fonctionne tout comme `History.last()`, mais en spécifiant
    /// le retour attendu.
    fn last_input(&self, input: &str) -> Result<Option<QueryDetailed>, Self::Error>;

    /// Ajoute le résultat d'une recherche dans le cache.
    fn push(&mut self, query: Query) -> Result<(), Self::Error>;
}
