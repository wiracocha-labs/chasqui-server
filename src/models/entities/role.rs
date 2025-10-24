
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use uuid::Uuid;
use std::fmt;
use std::str::FromStr;

/// Representa un rol en el sistema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Role {
    /// Identificador único del rol (SurrealDB Thing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    /// Nombre único del rol (ej: "admin", "moderator", "user")
    pub name: String,
    /// Descripción del rol
    pub description: String,
    /// Permisos asociados al rol
    pub permissions: Vec<Permission>,
}

/// Permisos disponibles en el sistema
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Permission {
    // Permisos de administración
    AdminAll,  // Acceso total al sistema
    
    // Permisos de workspace
    WorkspaceCreate,
    WorkspaceRead,
    WorkspaceUpdate,
    WorkspaceDelete,
    WorkspaceManageMembers,
    
    // Permisos de canal
    ChannelCreate,
    ChannelRead,
    ChannelUpdate,
    ChannelDelete,
    ChannelSendMessages,
    
    // Permisos de mensajes
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessagePin,
    
    // Permisos de usuario
    UserInvite,
    UserKick,
    UserBan,
}

impl Role {
    /// Crea un nuevo rol
    pub fn new(name: &str, description: &str) -> Self {
        let uuid = Uuid::new_v4().to_string();
        Self {
            id: Some(Thing::from(("role", uuid.as_str()))), // role:<uuid>
            name: name.to_string(),
            description: description.to_string(),
            permissions: Vec::new(),
        }
    }
    
    /// Añade permisos al rol
    pub fn with_permissions(mut self, permissions: &[Permission]) -> Self {
        self.permissions = permissions.to_vec();
        self
    }
    
    /// Verifica si el rol tiene un permiso específico
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions.contains(&permission) || self.permissions.contains(&Permission::AdminAll)
    }
}

// Implementación de Display para Permission
impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let permission_str = match self {
            Permission::AdminAll => "admin:all",
            Permission::WorkspaceCreate => "workspace:create",
            // ... otros permisos
            _ => "unknown",
        };
        write!(f, "{}", permission_str)
    }
}

// Implementación de FromStr para Permission
impl FromStr for Permission {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin:all" => Ok(Permission::AdminAll),
            "workspace:create" => Ok(Permission::WorkspaceCreate),
            // ... otros permisos
            _ => Err(format!("Permiso no válido: {}", s)),
        }
    }
}

// Tipos de roles predefinidos
pub mod roles {
    use super::*;
    
    /// Rol de administrador con todos los permisos
    pub fn admin() -> Role {
        Role::new("admin", "Administrador del sistema")
            .with_permissions(&[
                Permission::AdminAll,
            ])
    }
    
    /// Rol de moderador con permisos limitados
    pub fn moderator() -> Role {
        Role::new("moderator", "Moderador del sistema")
            .with_permissions(&[
                Permission::WorkspaceRead,
                Permission::ChannelRead,
                Permission::ChannelSendMessages,
                Permission::MessageDelete,
            ])
    }
    
    /// Rol de usuario estándar
    pub fn user() -> Role {
        Role::new("user", "Usuario estándar")
            .with_permissions(&[
                Permission::WorkspaceRead,
                Permission::ChannelRead,
                Permission::ChannelSendMessages,
            ])
    }
}