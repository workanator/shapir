use chrono::{DateTime, UTC};
use serde_json::Value;
use ::error::{Result, Error, IoError, IoErrorKind};
use super::{Kind, AccessRight};


/// Share Details
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Share {
    /// Share type: Send or Request.
    pub kind: Kind,
    /// Access rights for the share.
    pub access_right: AccessRight,
    /// ID of the share.
    pub id: String,
    /// Alias ID of the share.
    pub alias_id: String,
    /// Date the share was created.
    pub creation_date: DateTime<UTC>,
    /// Date the share expires.
    pub expiration_date: DateTime<UTC>,
    /// User activity on this share will be tracked up to this date.
    pub track_until_date: DateTime<UTC>,
    /// Maximum number of downloads each user can perform.
    pub max_downloads: i32,
    /// Total number of times a share has been downloaded by a user.
    pub total_downloads: i32,
    /// Subject of Share email message.
    pub sent_message_title: String,
    /// HMAC Signature for the share data.
    pub signature: String,
    /// Share title.
    pub title: String,
    /// Uri to access the share through the Web portal.
    pub uri: String,
    /// Indicates whether or not this share has been archived.
    pub is_archived: bool,
    /// Indicates whether or not this share has been downloaded.
    pub is_consumed: bool,
    /// Indicates whether the contents of this share have been viewed by a valid, authenticated recipient.
    pub is_read: bool,
    /// Indicates if the share is view only.
    pub is_view_only: bool,
    /// Indicates if only authenticated users can download files from this share.
    pub require_login: bool,
    /// Indicates if users must provide Name, Email and Company information to download files from the share.
    pub require_user_info: bool,
    /// Flag to indicate if ShareFile has sent email messages for this share.
    pub has_sent_message: bool,
    /// When enabled the items are identified by stream IDs instead of item IDs. Applies to Send Shares only.
    pub uses_stream_ids: bool,
}


impl Share {
    /// Construct share from the decoded JSON value.
    pub fn from_json(value: Value) -> Result<Share> {
        if let Some(object) = value.as_object() {
            // Read share type
            let kind = match object.get("ShareType") {
                Some(v) => match Kind::from_json(v.clone()) {
                    Ok(kind) => kind,
                    Err(e) => return Err(e),
                },
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.ShareType property is missing.")),
            };

            // Read access right
            let access_right = match object.get("ShareAccessRight") {
                Some(v) => match AccessRight::from_json(v.clone()) {
                    Ok(access_right) => access_right,
                    Err(e) => return Err(e),
                },
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.ShareAccessRight property is missing.")),
            };

            // Read ID
            let id = match object.get("Id") {
                Some(v) => v.as_str().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.Id property is missing.")),
            };

            // Read Alias ID
            let alias_id = match object.get("AliasID") {
                Some(v) => v.as_str().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.AliasID property is missing.")),
            };

            // Read creation date
            let creation_date = match object.get("CreationDate") {
                Some(v) => match v.as_str().unwrap().parse::<DateTime<UTC>>() {
                    Ok(dt) => dt,
                    Err(err) => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, format!("Share.CreationDate property is invalid because {}", err)))
                },
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.CreationDate property is missing.")),
            };

            // Read expiration date
            let expiration_date = match object.get("ExpirationDate") {
                Some(v) => match v.as_str().unwrap().parse::<DateTime<UTC>>() {
                    Ok(dt) => dt,
                    Err(err) => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, format!("Share.ExpirationDate property is invalid because {}", err)))
                },
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.ExpirationDate property is missing.")),
            };

            // Read track until date
            let track_until_date = match object.get("TrackUntilDate") {
                Some(v) => match v.as_str().unwrap().parse::<DateTime<UTC>>() {
                    Ok(dt) => dt,
                    Err(err) => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, format!("Share.TrackUntilDate property is invalid because {}", err)))
                },
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.TrackUntilDate property is missing.")),
            };

            // Read max. downloads
            let max_downloads = match value.find("MaxDownloads") {
                Some(v) => v.as_i64().unwrap() as i32,
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.MaxDownloads property is missing."))
            };

            // Read total downloads
            let total_downloads = match value.find("TotalDownloads") {
                Some(v) => v.as_i64().unwrap() as i32,
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.TotalDownloads property is missing."))
            };

            // Read sent message title
            let sent_message_title = match object.get("SentMessageTitle") {
                Some(v) => v.as_str().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.SentMessageTitle property is missing.")),
            };

            // Read signature
            let signature = match object.get("Signature") {
                Some(v) => v.as_str().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.Signature property is missing.")),
            };

            // Read title
            let title = match object.get("Title") {
                Some(v) => v.as_str().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.Title property is missing.")),
            };

            // Read URI
            let uri = match object.get("Uri") {
                Some(v) => v.as_str().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.Uri property is missing.")),
            };

            // Read archived flag (this property is probably optional)
            let is_archived = match object.get("IsArchived") {
                Some(v) => v.as_bool().unwrap(),
                None => false,
            };

            // Read consumed flag
            let is_consumed = match object.get("IsConsumed") {
                Some(v) => v.as_bool().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.IsConsumed property is missing.")),
            };

            // Read read flag
            let is_read = match object.get("IsRead") {
                Some(v) => v.as_bool().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.IsRead property is missing.")),
            };

            // Read view only flag (this property is probably optional)
            let is_view_only = match object.get("IsViewOnly") {
                Some(v) => v.as_bool().unwrap(),
                None => false,
            };

            // Read require login flag
            let require_login = match object.get("RequireLogin") {
                Some(v) => v.as_bool().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.RequireLogin property is missing.")),
            };

            // Read require user info flag
            let require_user_info = match object.get("RequireUserInfo") {
                Some(v) => v.as_bool().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.RequireUserInfo property is missing.")),
            };

            // Read message sent flag
            let has_sent_message = match object.get("HasSentMessage") {
                Some(v) => v.as_bool().unwrap(),
                None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share.HasSentMessage property is missing.")),
            };

            // Read uses stream IDs flag (this property is probably optional)
            let uses_stream_ids = match object.get("UsesStreamIDs") {
                Some(v) => v.as_bool().unwrap(),
                None => false,
            };


            // Construct the Share struct
            Ok(Share {
                kind: kind,
                access_right: access_right,
                id: id.to_owned(),
                alias_id: alias_id.to_owned(),
                creation_date: creation_date,
                expiration_date: expiration_date,
                track_until_date: track_until_date,
                max_downloads: max_downloads,
                total_downloads: total_downloads,
                sent_message_title: sent_message_title.to_owned(),
                signature: signature.to_owned(),
                title: title.to_owned(),
                uri: uri.to_owned(),
                is_archived: is_archived,
                is_consumed: is_consumed,
                is_read: is_read,
                is_view_only: is_view_only,
                require_login: require_login,
                require_user_info: require_user_info,
                has_sent_message: has_sent_message,
                uses_stream_ids: uses_stream_ids,
            })
        }
        else {
            Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Share can be constructed from JSON Object only."))
        }
    }
}
