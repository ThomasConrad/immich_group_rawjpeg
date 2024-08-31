/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.113.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "activity.create")]
    ActivityPeriodCreate,
    #[serde(rename = "activity.read")]
    ActivityPeriodRead,
    #[serde(rename = "activity.update")]
    ActivityPeriodUpdate,
    #[serde(rename = "activity.delete")]
    ActivityPeriodDelete,
    #[serde(rename = "activity.statistics")]
    ActivityPeriodStatistics,
    #[serde(rename = "apiKey.create")]
    ApiKeyPeriodCreate,
    #[serde(rename = "apiKey.read")]
    ApiKeyPeriodRead,
    #[serde(rename = "apiKey.update")]
    ApiKeyPeriodUpdate,
    #[serde(rename = "apiKey.delete")]
    ApiKeyPeriodDelete,
    #[serde(rename = "asset.read")]
    AssetPeriodRead,
    #[serde(rename = "asset.update")]
    AssetPeriodUpdate,
    #[serde(rename = "asset.delete")]
    AssetPeriodDelete,
    #[serde(rename = "asset.share")]
    AssetPeriodShare,
    #[serde(rename = "asset.view")]
    AssetPeriodView,
    #[serde(rename = "asset.download")]
    AssetPeriodDownload,
    #[serde(rename = "asset.upload")]
    AssetPeriodUpload,
    #[serde(rename = "album.create")]
    AlbumPeriodCreate,
    #[serde(rename = "album.read")]
    AlbumPeriodRead,
    #[serde(rename = "album.update")]
    AlbumPeriodUpdate,
    #[serde(rename = "album.delete")]
    AlbumPeriodDelete,
    #[serde(rename = "album.statistics")]
    AlbumPeriodStatistics,
    #[serde(rename = "album.addAsset")]
    AlbumPeriodAddAsset,
    #[serde(rename = "album.removeAsset")]
    AlbumPeriodRemoveAsset,
    #[serde(rename = "album.share")]
    AlbumPeriodShare,
    #[serde(rename = "album.download")]
    AlbumPeriodDownload,
    #[serde(rename = "authDevice.delete")]
    AuthDevicePeriodDelete,
    #[serde(rename = "archive.read")]
    ArchivePeriodRead,
    #[serde(rename = "face.create")]
    FacePeriodCreate,
    #[serde(rename = "face.read")]
    FacePeriodRead,
    #[serde(rename = "face.update")]
    FacePeriodUpdate,
    #[serde(rename = "face.delete")]
    FacePeriodDelete,
    #[serde(rename = "library.create")]
    LibraryPeriodCreate,
    #[serde(rename = "library.read")]
    LibraryPeriodRead,
    #[serde(rename = "library.update")]
    LibraryPeriodUpdate,
    #[serde(rename = "library.delete")]
    LibraryPeriodDelete,
    #[serde(rename = "library.statistics")]
    LibraryPeriodStatistics,
    #[serde(rename = "timeline.read")]
    TimelinePeriodRead,
    #[serde(rename = "timeline.download")]
    TimelinePeriodDownload,
    #[serde(rename = "memory.create")]
    MemoryPeriodCreate,
    #[serde(rename = "memory.read")]
    MemoryPeriodRead,
    #[serde(rename = "memory.update")]
    MemoryPeriodUpdate,
    #[serde(rename = "memory.delete")]
    MemoryPeriodDelete,
    #[serde(rename = "partner.create")]
    PartnerPeriodCreate,
    #[serde(rename = "partner.read")]
    PartnerPeriodRead,
    #[serde(rename = "partner.update")]
    PartnerPeriodUpdate,
    #[serde(rename = "partner.delete")]
    PartnerPeriodDelete,
    #[serde(rename = "person.create")]
    PersonPeriodCreate,
    #[serde(rename = "person.read")]
    PersonPeriodRead,
    #[serde(rename = "person.update")]
    PersonPeriodUpdate,
    #[serde(rename = "person.delete")]
    PersonPeriodDelete,
    #[serde(rename = "person.statistics")]
    PersonPeriodStatistics,
    #[serde(rename = "person.merge")]
    PersonPeriodMerge,
    #[serde(rename = "person.reassign")]
    PersonPeriodReassign,
    #[serde(rename = "session.read")]
    SessionPeriodRead,
    #[serde(rename = "session.update")]
    SessionPeriodUpdate,
    #[serde(rename = "session.delete")]
    SessionPeriodDelete,
    #[serde(rename = "sharedLink.create")]
    SharedLinkPeriodCreate,
    #[serde(rename = "sharedLink.read")]
    SharedLinkPeriodRead,
    #[serde(rename = "sharedLink.update")]
    SharedLinkPeriodUpdate,
    #[serde(rename = "sharedLink.delete")]
    SharedLinkPeriodDelete,
    #[serde(rename = "stack.create")]
    StackPeriodCreate,
    #[serde(rename = "stack.read")]
    StackPeriodRead,
    #[serde(rename = "stack.update")]
    StackPeriodUpdate,
    #[serde(rename = "stack.delete")]
    StackPeriodDelete,
    #[serde(rename = "systemConfig.read")]
    SystemConfigPeriodRead,
    #[serde(rename = "systemConfig.update")]
    SystemConfigPeriodUpdate,
    #[serde(rename = "systemMetadata.read")]
    SystemMetadataPeriodRead,
    #[serde(rename = "systemMetadata.update")]
    SystemMetadataPeriodUpdate,
    #[serde(rename = "tag.create")]
    TagPeriodCreate,
    #[serde(rename = "tag.read")]
    TagPeriodRead,
    #[serde(rename = "tag.update")]
    TagPeriodUpdate,
    #[serde(rename = "tag.delete")]
    TagPeriodDelete,
    #[serde(rename = "tag.asset")]
    TagPeriodAsset,
    #[serde(rename = "admin.user.create")]
    AdminPeriodUserPeriodCreate,
    #[serde(rename = "admin.user.read")]
    AdminPeriodUserPeriodRead,
    #[serde(rename = "admin.user.update")]
    AdminPeriodUserPeriodUpdate,
    #[serde(rename = "admin.user.delete")]
    AdminPeriodUserPeriodDelete,

}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::ActivityPeriodCreate => write!(f, "activity.create"),
            Self::ActivityPeriodRead => write!(f, "activity.read"),
            Self::ActivityPeriodUpdate => write!(f, "activity.update"),
            Self::ActivityPeriodDelete => write!(f, "activity.delete"),
            Self::ActivityPeriodStatistics => write!(f, "activity.statistics"),
            Self::ApiKeyPeriodCreate => write!(f, "apiKey.create"),
            Self::ApiKeyPeriodRead => write!(f, "apiKey.read"),
            Self::ApiKeyPeriodUpdate => write!(f, "apiKey.update"),
            Self::ApiKeyPeriodDelete => write!(f, "apiKey.delete"),
            Self::AssetPeriodRead => write!(f, "asset.read"),
            Self::AssetPeriodUpdate => write!(f, "asset.update"),
            Self::AssetPeriodDelete => write!(f, "asset.delete"),
            Self::AssetPeriodShare => write!(f, "asset.share"),
            Self::AssetPeriodView => write!(f, "asset.view"),
            Self::AssetPeriodDownload => write!(f, "asset.download"),
            Self::AssetPeriodUpload => write!(f, "asset.upload"),
            Self::AlbumPeriodCreate => write!(f, "album.create"),
            Self::AlbumPeriodRead => write!(f, "album.read"),
            Self::AlbumPeriodUpdate => write!(f, "album.update"),
            Self::AlbumPeriodDelete => write!(f, "album.delete"),
            Self::AlbumPeriodStatistics => write!(f, "album.statistics"),
            Self::AlbumPeriodAddAsset => write!(f, "album.addAsset"),
            Self::AlbumPeriodRemoveAsset => write!(f, "album.removeAsset"),
            Self::AlbumPeriodShare => write!(f, "album.share"),
            Self::AlbumPeriodDownload => write!(f, "album.download"),
            Self::AuthDevicePeriodDelete => write!(f, "authDevice.delete"),
            Self::ArchivePeriodRead => write!(f, "archive.read"),
            Self::FacePeriodCreate => write!(f, "face.create"),
            Self::FacePeriodRead => write!(f, "face.read"),
            Self::FacePeriodUpdate => write!(f, "face.update"),
            Self::FacePeriodDelete => write!(f, "face.delete"),
            Self::LibraryPeriodCreate => write!(f, "library.create"),
            Self::LibraryPeriodRead => write!(f, "library.read"),
            Self::LibraryPeriodUpdate => write!(f, "library.update"),
            Self::LibraryPeriodDelete => write!(f, "library.delete"),
            Self::LibraryPeriodStatistics => write!(f, "library.statistics"),
            Self::TimelinePeriodRead => write!(f, "timeline.read"),
            Self::TimelinePeriodDownload => write!(f, "timeline.download"),
            Self::MemoryPeriodCreate => write!(f, "memory.create"),
            Self::MemoryPeriodRead => write!(f, "memory.read"),
            Self::MemoryPeriodUpdate => write!(f, "memory.update"),
            Self::MemoryPeriodDelete => write!(f, "memory.delete"),
            Self::PartnerPeriodCreate => write!(f, "partner.create"),
            Self::PartnerPeriodRead => write!(f, "partner.read"),
            Self::PartnerPeriodUpdate => write!(f, "partner.update"),
            Self::PartnerPeriodDelete => write!(f, "partner.delete"),
            Self::PersonPeriodCreate => write!(f, "person.create"),
            Self::PersonPeriodRead => write!(f, "person.read"),
            Self::PersonPeriodUpdate => write!(f, "person.update"),
            Self::PersonPeriodDelete => write!(f, "person.delete"),
            Self::PersonPeriodStatistics => write!(f, "person.statistics"),
            Self::PersonPeriodMerge => write!(f, "person.merge"),
            Self::PersonPeriodReassign => write!(f, "person.reassign"),
            Self::SessionPeriodRead => write!(f, "session.read"),
            Self::SessionPeriodUpdate => write!(f, "session.update"),
            Self::SessionPeriodDelete => write!(f, "session.delete"),
            Self::SharedLinkPeriodCreate => write!(f, "sharedLink.create"),
            Self::SharedLinkPeriodRead => write!(f, "sharedLink.read"),
            Self::SharedLinkPeriodUpdate => write!(f, "sharedLink.update"),
            Self::SharedLinkPeriodDelete => write!(f, "sharedLink.delete"),
            Self::StackPeriodCreate => write!(f, "stack.create"),
            Self::StackPeriodRead => write!(f, "stack.read"),
            Self::StackPeriodUpdate => write!(f, "stack.update"),
            Self::StackPeriodDelete => write!(f, "stack.delete"),
            Self::SystemConfigPeriodRead => write!(f, "systemConfig.read"),
            Self::SystemConfigPeriodUpdate => write!(f, "systemConfig.update"),
            Self::SystemMetadataPeriodRead => write!(f, "systemMetadata.read"),
            Self::SystemMetadataPeriodUpdate => write!(f, "systemMetadata.update"),
            Self::TagPeriodCreate => write!(f, "tag.create"),
            Self::TagPeriodRead => write!(f, "tag.read"),
            Self::TagPeriodUpdate => write!(f, "tag.update"),
            Self::TagPeriodDelete => write!(f, "tag.delete"),
            Self::TagPeriodAsset => write!(f, "tag.asset"),
            Self::AdminPeriodUserPeriodCreate => write!(f, "admin.user.create"),
            Self::AdminPeriodUserPeriodRead => write!(f, "admin.user.read"),
            Self::AdminPeriodUserPeriodUpdate => write!(f, "admin.user.update"),
            Self::AdminPeriodUserPeriodDelete => write!(f, "admin.user.delete"),
        }
    }
}

impl Default for Permission {
    fn default() -> Permission {
        Self::All
    }
}

