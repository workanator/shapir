use super::Kind;


/// Share configuration
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShareConfig {
	kind: Kind,
}

/*
		ShareType => $type,
		Title => $title,
		Items => $id,
		Recipients => $recipient,
		ExpirationDate => time2str('%Y-%m-%d', $expires_on),
		RequireLogin => $require_login ? Types::Serialiser::true : Types::Serialiser::false,
		RequireUserInfo => $require_user_info ? Types::Serialiser::true : Types::Serialiser::false,
		MaxDownloads => $max_downloads,
		UsesStreamIDs => Types::Serialiser::false
*/