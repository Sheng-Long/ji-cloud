//! Types for images.

use super::category::CategoryId;
use super::meta::{AffiliationId, AgeRangeId, StyleId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "backend")]
use sqlx::postgres::PgRow;
use url::Url;
use uuid::Uuid;

/// Wrapper type around [`Uuid`], represents the ID of a image.
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct ImageId(pub Uuid);

/// Represents when to publish an image.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Publish {
    /// Publish the image *at* the given time.
    At(DateTime<Utc>),
    /// Publish the image *in* the given amount of time from now.
    In(std::time::Duration),
}

impl Publish {
    /// creates an instance of `Self` that will publish "right now"
    pub fn now() -> Self {
        Self::In(std::time::Duration::new(0, 0))
    }
}

impl From<Publish> for DateTime<Utc> {
    fn from(publish: Publish) -> Self {
        match publish {
            Publish::At(t) => t,
            Publish::In(d) => {
                // todo: error instead of panicing
                Utc::now() + chrono::Duration::from_std(d).expect("Really really big duration?")
            }
        }
    }
}

// todo: # errors doc section
/// Request to create a new image.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRequest {
    /// The name of the image.
    pub name: String,
    /// The description of the image.
    pub description: String,
    /// Is the image premium?
    pub is_premium: bool,
    /// When to publish the image.
    ///
    /// If [`Some`] publish the image according to the `Publish`. Otherwise, don't publish it.
    pub publish_at: Option<Publish>,
    /// The image's styles.
    pub styles: Vec<StyleId>,
    /// The image's age ranges.
    pub age_ranges: Vec<AgeRangeId>,
    /// The image's affiliations.
    pub affiliations: Vec<AffiliationId>,
    /// The image's categories.
    pub categories: Vec<CategoryId>,
}

// todo: # errors doc section.
#[derive(Serialize, Deserialize, Debug, Default)]
/// Request to update an image.
///
/// All fields are optional, any field that is [`None`] will not be updated.
pub struct UpdateRequest {
    /// If `Some` change the image's name to this name.
    pub name: Option<String>,

    /// If `Some` change the image's description to this description.
    pub description: Option<String>,

    /// If `Some` mark the image as premium or not.
    pub is_premium: Option<bool>,

    /// If `Some`, change the `publish_at` to the given `Option<Publish>`.
    ///
    /// Specifically, if `None`, don't update.
    /// If `Some(None)`, set the `publish_at` to `None`, unpublishing it if previously published.
    /// Otherwise set it to the given [`Publish`].
    ///
    /// [`Publish`]: struct.Publish.html
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub publish_at: Option<Option<Publish>>,

    /// If `Some` replace the image's styles with these.
    pub styles: Option<Vec<StyleId>>,

    /// If `Some` replace the image's age ranges with these.
    pub age_ranges: Option<Vec<AgeRangeId>>,

    /// If `Some` replace the image's affiliations with these.
    pub affiliations: Option<Vec<AffiliationId>>,

    /// If `Some` replace the image's categories with these.
    pub categories: Option<Vec<CategoryId>>,
}

/// Search for images via the given query string.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SearchQuery {
    /// The query string.
    pub q: String,
}

/// Response for successful search.
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    /// the images returned.
    pub images: Vec<GetResponse>,
}

/// Response for getting a single image.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponse {
    /// The image metadata.
    pub metadata: Image,
    /// A url that can be used to retrieve the image.
    pub url: Url,
}

/// Over the wire representation of an image's metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    /// The image's ID.
    pub id: ImageId,

    /// The name of the image.
    pub name: String,

    /// A string describing the image.
    pub description: String,

    /// Whether or not the image is premium.
    pub is_premium: bool,

    /// When the image should be considered published (if at all).
    pub publish_at: Option<DateTime<Utc>>,

    /// The styles associated with the image.
    pub styles: Vec<StyleId>,

    /// The age ranges associated with the image.
    pub age_ranges: Vec<AgeRangeId>,

    /// The affiliations associated with the image.
    pub affiliations: Vec<AffiliationId>,

    /// The categories associated with the image.
    pub categories: Vec<CategoryId>,

    /// When the image was originally created.
    pub created_at: DateTime<Utc>,

    /// When the image was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Response for successfully creating an image.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResponse {
    /// The ID of the newly created image.
    pub id: ImageId,

    /// The URL to upload the image data to.
    pub upload_url: Url,
}

// HACK: we can't get `Vec<_>` directly from the DB, so we have to work around it for now.
// see: https://github.com/launchbadge/sqlx/issues/298
#[cfg(feature = "backend")]
impl<'r> sqlx::FromRow<'r, PgRow> for Image {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let DbImage {
            id,
            name,
            description,
            is_premium,
            publish_at,
            styles,
            age_ranges,
            affiliations,
            categories,
            created_at,
            updated_at,
        } = DbImage::from_row(row)?;

        Ok(Self {
            id,
            name,
            description,
            is_premium,
            publish_at,
            styles: styles.into_iter().map(|(it,)| it).collect(),
            age_ranges: age_ranges.into_iter().map(|(it,)| it).collect(),
            affiliations: affiliations.into_iter().map(|(it,)| it).collect(),
            categories: categories.into_iter().map(|(it,)| it).collect(),
            created_at,
            updated_at,
        })
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[cfg(feature = "backend")]
struct DbImage {
    pub id: ImageId,
    pub name: String,
    pub description: String,
    pub is_premium: bool,
    pub publish_at: Option<DateTime<Utc>>,
    pub styles: Vec<(StyleId,)>,
    pub age_ranges: Vec<(AgeRangeId,)>,
    pub affiliations: Vec<(AffiliationId,)>,
    pub categories: Vec<(CategoryId,)>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
