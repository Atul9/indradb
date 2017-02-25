use traits::Id;
use regex::Regex;
use errors::ValidationError;
use core::str::FromStr;
use chrono::{UTC, DateTime};

lazy_static! {
	static ref TYPE_VALIDATOR: Regex = Regex::new("^[a-zA-Z0-9-_]+$").unwrap();
}

/// A vertex.
///
/// Vertices are how you would represent nouns in the datastore. An example
/// might be a user, or a movie. All vertices have a unique ID and a type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertex<I: Id> {
    /// The id of the vertex.
    pub id: I,

    /// The type of the vertex.
    #[serde(rename="type")]
    pub t: Type,
}

impl<I: Id> Vertex<I> {
    /// Creates a new vertex.
    ///
    /// # Arguments
    /// 
    /// * `id` - The id of the vertex.
    /// * `t` - The type of the vertex.
    pub fn new(id: I, t: Type) -> Vertex<I> {
        Vertex { id: id, t: t }
    }
}

impl<I: Id> PartialEq for Vertex<I> {
    fn eq(&self, other: &Vertex<I>) -> bool {
        self.id == other.id
    }
}

impl<I: Id> Eq for Vertex<I> {}

/// An edge.
///
/// Edges are how you would represent a verb or a relationship in the
/// datastore. An example might be "liked" or "reviewed". Edges are typed,
/// weighted and directed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge<I: Id> {
    /// The id of the outbound vertex.
    pub outbound_id: I,
    #[serde(rename="type")]

    /// The type of the edge.
    pub t: Type,

    /// The id of the inbound vertex.
    pub inbound_id: I,

    /// The weight of the edge.
    pub weight: Weight,

    /// When the edge was last updated.
    pub update_datetime: DateTime<UTC>
}

impl<I: Id> Edge<I> {
    /// Creates a new edge with the current datetime in UTC.
    ///
    /// # Arguments
    /// 
    /// * `outbound_id` - The id of the outbound vertex.
    /// * `t` - The type of the edge.
    /// * `inbound_id` - The id of the inbound vertex.
    /// * `weight` - The weight of the edge.
    pub fn new_with_current_datetime(outbound_id: I, t: Type, inbound_id: I, weight: Weight) -> Edge<I> {
        Self::new(outbound_id, t, inbound_id, weight, UTC::now())
    }

    /// Creates a new edge with a specified datetime.
    ///
    /// # Arguments
    /// 
    /// * `outbound_id` - The id of the outbound vertex.
    /// * `t` - The type of the edge.
    /// * `inbound_id` - The id of the inbound vertex.
    /// * `weight` - The weight of the edge.
    /// * `update_datetime` - When the edge was last updated.
    pub fn new(outbound_id: I, t: Type, inbound_id: I, weight: Weight, update_datetime: DateTime<UTC>) -> Edge<I> {
        Edge {
            outbound_id: outbound_id,
            t: t,
            inbound_id: inbound_id,
            weight: weight,
            update_datetime: update_datetime
        }
    }
}

impl<I: Id> PartialEq for Edge<I> {
    fn eq(&self, other: &Edge<I>) -> bool {
        self.outbound_id == other.outbound_id && self.t == other.t &&
        self.inbound_id == other.inbound_id
    }
}

impl<I: Id> Eq for Edge<I> {}

/// An edge weight.
///
/// Edge weights must be between -1.0 and 1.0.
#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct Weight(pub f32);

impl Weight {
    /// Constructs a new edge weight.
    ///
    /// # Arguments
    /// 
    /// * `weight` - The weight, between -1.0 and 1.0.
    ///
    /// # Errors
    /// Returns a `ValidationError` if the weight is below -1.0 or above 1.0.
    pub fn new(w: f32) -> Result<Self, ValidationError> {
        if w < -1.0 || w > 1.0 {
            Err(ValidationError::new("Weight out of range".to_string()))
        } else {
            Ok(Weight(w))
        }
    }
}

/// An edge or vertex type.
///
/// Types must be less than 256 characters long, and can only contain letters,
/// numbers, dashes and underscores.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub struct Type(pub String);

impl Type {
    /// Constructs a new type.
    ///
    /// # Arguments
    /// 
    /// * `t` - The type, which must be less than 256 characters long.
    ///
    /// # Errors
    /// Returns a `ValidationError` if the type is longer than 255 characters,
    /// or has invalid characters.
    pub fn new(t: String) -> Result<Self, ValidationError> {
        if t.len() > 255 {
            Err(ValidationError::new("Type is too long".to_string()))
        } else if !TYPE_VALIDATOR.is_match(&t[..]) {
            Err(ValidationError::new("Invalid type".to_string()))
        } else {
            Ok(Type(t))
        }
    }
}

impl FromStr for Type {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string())?)
    }
}
