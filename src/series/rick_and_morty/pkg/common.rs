use wtx::misc::Vector;

pub(crate) const CHARACTER_FRAGMENT: &str = r#"
  fragment CharacterFrag on Character {
    created
    gender
    id
    image
    location {
      created
      dimension
      id
      name
      type
    }
    name
    origin {
      created
      dimension
      id
      name
      type
    }
    species
    status
    type
  }
"#;

/// Character
#[derive(Debug, serde::Deserialize)]
pub struct Character<T> {
  /// Time at which the character was created in the database.
  pub created: T,
  /// Episodes in which this character appeared.
  pub episode: Option<Vector<Episode<T>>>,
  /// The gender of the character ('Female', 'Male', 'Genderless' or 'unknown').
  pub gender: T,
  /// The id of the character.
  pub id: T,
  /// Link to the character's image.
  pub image: T,
  /// The character's last known location
  pub location: Location<T>,
  /// The name of the character.
  pub name: T,
  /// The character's origin location
  pub origin: Location<T>,
  /// The species of the character.
  pub species: T,
  /// The status of the character ('Alive', 'Dead' or 'unknown').
  pub status: T,
  /// The type or subspecies of the character.
  pub r#type: T,
}

/// Episode
#[derive(Debug, serde::Deserialize)]
pub struct Episode<T> {
  /// The air date of the episode.
  pub air_date: T,
  /// List of characters who have been seen in the episode.
  pub characters: Vector<Character<T>>,
  /// Time at which the episode was created in the database.
  pub created: T,
  /// The code of the episode.
  pub episode: T,
  /// The id of the episode.
  pub id: T,
  /// The name of the episode.
  pub name: T,
}

/// Location
#[derive(Debug, serde::Deserialize)]
pub struct Location<T> {
  /// Time at which the location was created in the database.
  pub created: Option<T>,
  /// The dimension in which the location is located.
  pub dimension: Option<T>,
  /// The id of the location.
  pub id: Option<T>,
  /// The name of the location.
  pub name: T,
  /// List of characters who have been last seen in the location.
  pub residents: Option<Vector<Character<T>>>,
  /// The type of the location.
  pub r#type: Option<T>,
}

/// Pagination
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Pagination<T> {
  /// Total number of entries
  pub count: u32,
  /// Next page
  pub next: Option<u32>,
  /// Total number of pages
  pub pages: u32,
  /// Previous page
  pub prev: Option<T>,
}
