use crate::models::User;
use async_graphql::{async_trait, guard::Guard};
use async_graphql::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    CreateCoffee,
    UpdateCoffee,
    DeleteCoffee,
    Other(String),
}

/*
impl Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            Permission::CreateCoffee => "create:coffee",
            Permission::UpdateCoffee => "update:coffee",
            Permission::DeleteCoffee => "delete:coffee",
            Permission::Other(ref other) => other,
        })
    }
}

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "create:waves" => Permission::CreateWaves,
            "create:battle_cards" => Permission::CreateBattleCards,
            _ => Permission::Other(s),
        })
    }
}
*/

pub struct PermissionGuard {
    pub permission: Permission,
}

#[async_trait::async_trait]
impl Guard for PermissionGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        // let maybe_user = ctx.data_opt::<User>();
        if let Some(user) = ctx.data_opt::<User>() {
            if user.has_permission(&self.permission).await {
                Ok(())
            } else {
                Err("Invalid permissions".into())
            }
        } else {
            Err("Must be authenticated".into())
        }
    }
}