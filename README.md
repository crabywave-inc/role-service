<p align="center">
  <a href="https://github.com/crabywave-inc/role-service">
    <img src="https://raw.githubusercontent.com/crabywave-inc/public-resources/main/role-service-banner.webp" width="200px" alt="Kanri" />
  </a>
</p>

<h3 align="center">Role Service</h3>
<p align="center">The Rust microservice to manage the roles guilds</p>

<div align="center">

![][rust-image]
[![license-image]][license-url]

</div>



<br />


## Overview

The Role Service is a microservice in the CrabWave project responsible for managing user roles and permissions within the platform. This service ensures granular access control, enabling administrators to define and enforce permissions at the workspace and channel levels. The primary focus is to manage roles within guilds, providing a flexible and scalable role management system.

---

## Objectives
- **Role Management**: Provide APIs to create, read, update, and delete roles within guilds.
- **Integration**: Seamlessly integrate with other microservicse, such as Auth and User Services.
- **Scalability**: Handle growth in users, guilds, and roles without performance degradation.
- **Security**: Ensure all role and permission operations are secure and auditable.

---

## Scope

The Role Service covers the following functionalities:
- Role creation and management within guilds.
- Permission assignment to roles.
- Retrieval of roles and associated permissions.
- Listening to topics to update collections for guild members (e.g., when a member is created, assigned a new role, or has a role removed).

It does not:
- Directly handle authentication (delegated to the Auth Service).
- Manage user accounts (handled by the User Service).

---

## Architecture

### Components

1. API Layer:
    - Framework: Rust with Axum
    - Endpoints:
      - **POST** `/guilds/{guild.id}/roles` - Create a new role in a guild.
      - **GET** `/guilds/{guild.id}/roles` - Retrieve all roles in a guild.
      - **GET** `/guilds/{guild.id}/roles/{role.id}` - Retrieve a specific role in a guild.
      - **PUT** `/guilds/{guild.id}/roles/{role.id}` - Update a role in a guild.
      - **DELETE** `/guilds/{guild.id}/roles/{role.id}` - Delete a role in a guild.
2. Service Layer
    - Business logic for role and permission management within guilds.
    - Integration with other services via Messaging Broker (e.g., PubSub).
3. Database
   - Firestore (NoSQL database)
   - Collections:
     - `roles`: stores role definitions for each guild.
     - `members`: stores role assignments for each member in a guild.
4. Event System
    - PubSub for listening to events from other services (e.g., User Service, Member Service).
    - Subscriptions:
      - `member-created-role`
      - `member-roles-added-role`
      - `member-roles-removed-role`
5. Authentication Middleware
    - Validate JWT tokens from incoming requests.

---

## Technology Stack
- Programming Language: Rust
- Framework: Axum
- Database Firestore (NoSQL)
- Message Broker: Google PubSub
- Deployment: Google Cloud Run
- CI/CD: Google Cloud Build

---

## API Design

### Example Endpoints

**Create Role**
**Request**

```json
POST /guilds/{guild.id}/roles
{
  "name": "Moderator",
  "permissions": "5412", // Permission bitmask
  "color": "#FF0000",
  "hoist": true,
  "mentionable": true
}
```

**Response**

```json
201 Created

{
   "status": 201,
   "data": {
      "id": "123",
      "name": "Moderator",
      "guild_id": "456",
      "color": "#FF0000",
      "position": 1,
      "permissions": "5412",
      "hoist": true,
      "mentionable": true
   }
}
```

---

## Conclusion

The Role Service is a critical component for maintaining secure and flexible access control in the CrabWave ecosystem. By adhering to best practices in microservice
design, this service will ensure scalability, security, and seamless integration with platform.


[rust-image]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[license-url]: LICENSE.md
[license-image]: https://img.shields.io/badge/License-Apache_2.0-196f3d?style=for-the-badge&logo=apache&logoColor=white