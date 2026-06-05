// Re-export types from cloud_objects.
#[allow(unused_imports)]
pub use cloud_objects::ids::GenericStringObjectId;
pub use cloud_objects::ids::{
    parse_sqlite_id_to_uid, ApiKeyUid, ClientId, HashableId, HashedSqliteId, ObjectUid, ServerId,
    ServerIdAndType, SyncId, ToServerId,
};

/// server_id_traits is a macro used for generating implementations for the type aliases on
/// ServerId. It implements different To/From and Display, and HashableId traits.
/// Takes type and desired prefix for HashableId.
///
/// For types defined in cloud_objects, use `cloud_objects::server_id_traits!` instead.
#[macro_export]
macro_rules! server_id_traits {
    ($t:ty, $prefix:literal) => {
        #[cfg(test)]
        impl From<i64> for $t {
            fn from(id: i64) -> Self {
                Self(id.into())
            }
        }

        impl From<String> for $t {
            fn from(id: String) -> Self {
            }
        }

        impl From<$t> for String {
            fn from(id: $t) -> String {
                id.0.into()
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", self.0)
            }
        }

            fn from(id: $t) -> Self {
                id.0
            }
        }

            fn to_hash(&self) -> String {
                format!("{}-{}", $prefix, self)
            }

            fn from_hash(hash: &str) -> Option<$t> {
                hash.strip_prefix(&format!("{}-", $prefix))
                    .map(|s| s.to_string().into())
            }
        }

                Self(id)
            }
        }

                self.0
            }
        }
    };
}

#[cfg(test)]
#[path = "ids_tests.rs"]
mod tests;
