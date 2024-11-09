//! System parameters for accessing data from template-based items.

use bevy::{
    ecs::{
        query::{QueryData, QueryEntityError, WorldQuery},
        system::SystemParam,
    },
    prelude::*,
};

use crate::components::{Item, Template};

/// System parameter for accessing data from template-based items.
#[derive(SystemParam)]
pub struct ItemData<'w, 's, D: QueryData + 'static> {
    query: Query<'w, 's, (Option<&'static Template>, Option<D>), With<Item>>,
}

impl<'w, 's, D: QueryData> ItemData<'w, 's, D> {
    fn extend_find(&self, mut entity: Entity) -> Result<Option<Entity>, QueryEntityError> {
        loop {
            let (maybe_extends, maybe_data) = self.query.get(entity)?;
            if maybe_data.is_some() {
                break Ok(Some(entity));
            }
            let Some(&Template(new_entity)) = maybe_extends else {
                break Ok(None);
            };
            entity = new_entity;
        }
    }

    /// Access this item's components immutably if available or fallaback to template's components.
    pub fn extended_get(
        &self,
        entity: Entity,
    ) -> Result<Option<<D::ReadOnly as WorldQuery>::Item<'_>>, QueryEntityError> {
        let Some(entity) = self.extend_find(entity)? else {
            return Ok(None);
        };
        Ok(self.query.get(entity).unwrap().1)
    }

    /// Access this item's components immutably.
    pub fn get(
        &self,
        entity: Entity,
    ) -> Result<Option<<D::ReadOnly as WorldQuery>::Item<'_>>, QueryEntityError> {
        Ok(self.query.get(entity)?.1)
    }

    /// Access this item's components mutably.
    pub fn get_mut(&mut self, entity: Entity) -> Result<Option<D::Item<'_>>, QueryEntityError> {
        Ok(self.query.get_mut(entity)?.1)
    }
}
