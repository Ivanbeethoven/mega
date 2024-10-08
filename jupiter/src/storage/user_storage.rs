use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    QueryFilter,
};

use callisto::{ssh_keys, user};
use common::{errors::MegaError, utils::generate_id};

#[derive(Clone)]
pub struct UserStorage {
    pub connection: Arc<DatabaseConnection>,
}

impl UserStorage {
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn new(connection: Arc<DatabaseConnection>) -> Self {
        UserStorage { connection }
    }

    pub fn mock() -> Self {
        UserStorage {
            connection: Arc::new(DatabaseConnection::default()),
        }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<user::Model>, MegaError> {
        let res = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(self.get_connection())
            .await?;
        Ok(res)
    }

    pub async fn save_user(&self, user: user::Model) -> Result<(), MegaError> {
        let a_model = user.into_active_model();
        a_model.insert(self.get_connection()).await.unwrap();
        Ok(())
    }

    pub async fn save_ssh_key(
        &self,
        user_id: i64,
        title: &str,
        ssh_key: &str,
        finger: &str,
    ) -> Result<(), MegaError> {
        let model = ssh_keys::Model {
            id: generate_id(),
            user_id,
            title: title.to_owned(),
            ssh_key: ssh_key.to_owned(),
            finger: finger.to_owned(),
            created_at: chrono::Utc::now().naive_utc(),
        };
        let a_model = model.into_active_model();
        a_model.insert(self.get_connection()).await.unwrap();
        Ok(())
    }

    pub async fn list_user_ssh(&self, user_id: i64) -> Result<Vec<ssh_keys::Model>, MegaError> {
        let res = ssh_keys::Entity::find()
            .filter(ssh_keys::Column::UserId.eq(user_id))
            .all(self.get_connection())
            .await?;
        Ok(res)
    }

    pub async fn delete_ssh_key(&self, user_id: i64, id: i64) -> Result<(), MegaError> {
        let res = ssh_keys::Entity::find()
            .filter(ssh_keys::Column::Id.eq(id))
            .filter(ssh_keys::Column::UserId.eq(user_id))
            .one(self.get_connection())
            .await?;
        if let Some(model) = res {
            model.delete(self.get_connection()).await?;
        }
        Ok(())
    }

    pub async fn search_ssh_key_finger(
        &self,
        finger_print: &str,
    ) -> Result<Vec<ssh_keys::Model>, MegaError> {
        let res = ssh_keys::Entity::find()
            .filter(ssh_keys::Column::Finger.eq(finger_print))
            .all(self.get_connection())
            .await?;
        Ok(res)
    }
}
