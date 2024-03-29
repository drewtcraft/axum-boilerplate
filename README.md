# Rust Server Boilerplate
A template for starting new, lean, beautiful rust backends.

## Technologies
- Axum as the server framework with various Tower dependencies
- Askana for HTML templating
- SCSS for styling
- htmx for front end reactivity
- also... Alpine.js for front end stuff (I guess I'm imagining more complex behaviors, like clicking one field on a form disables another piece of it (don't need htmx for that!)) (can also set up some alpine stores in the base template if some sort of state needs to be maintained)
- sqlx + sqlite for a simple database

## Structure
Inspired by Django and Rails, the business logic is broken up into "apps", which are high-level namespaces that often have associated database models, client views,etc. 

Full structure:
- src/ -- all app code
  - router.rs -- all routes
  - database.rs -- database connection utilities
  - error.rs -- crate-level errors, re-export of Result
  - templates/ -- global views and partials including .html and .scss files
  - style.scss -- global styles (TODO: move into templates)
  - apps/ -- apps and related files
    - example_app/ -- all business logic for example_app
      - handlers.rs -- axum handlers for example_app
      - layers.rs -- axum layers for example_app
      - models.rs -- example_app "models", including methods for interacting with database
      - routes.rs -- routes for some or all handlers, and some or no layers
      - templates/ -- example_app views and partials including .html and .scss files
    - example_app_2_etc/
- migrations/ -- sqlite migration files
- data/ -- sqlite database files
- public/ -- static files
  - css/
  - js/
- build.rs -- build script, executed on cargo run|build
- askama.toml -- configuration for views and partials

## Adding a New App
TODO: add a generator for this sort of thing?
- add the app in `apps/<app_name>`
- add `pub mod <app_name>` to `apps/mod.rs`
- IF there are HTML templates, add the path to the templates file to the `dirs` array in `askama.toml`. Note that this sucks and is the main reason I'm considering adding some sort of generator to this... 
- models:
  - models.rs or models/ should contain one module per database table
    - each module should share the database table name and contain the structs and methods required to interact with the database, including input sanitization
      - e.g. mod user has...
        - a User struct representing a normal database row
        - a create_user method, which uses the User struct to create a new user
        - among other methods
        - if we needed a UserUpdate struct with a primary_key and some other data we would create that here as well 

## Running stuff
- `cargo run` will get things moving
- `cargo watch -w src -x run` will rerun on changes to `src/`  (BATTERY KILLER)

## Adding a new Table
- run `sqlx migrate add <migration_name>`
- open the newly generated migration file and write some SQLite!
- migrations run automatically on app startup

## Architecture at a glance
- USER
  - handlers
    - get_/post_sign_up
    - get_/post_log_in
    - log_out
    - get_/post_send_invite
    - get_cookie
    - admin_list_users
    - admin_get_/admin_post_edit_user
  - browser templates
    - log-in
    - log-out
    - sign-up
    - send-invite
    - admin_user-edit
    - admin_user-list
  - email templates
    - email_invite
    - email_log-in
  - middleware
    - restrict_to_user
    - pull_user_id_from_session_uid
  - models
    - user_roles
      - name -- god, admin, contributor, spectator
    - users
      - username
      - email
      - role_id FK(user_roles)
      - active
      - created_at
      - updated_at
    - user_temp_uid_purposes
      - name -- sign-up, log-in, session
    - user_temp_uids
      - uid
      - expires_at
      - purpose_id FK(user_temp_uid_purposes)
      - created_at
      - user_id

- FORUM
  - handlers
  - browser templates
  - models
    - file_extensions
      - extension -- png, jpg, jpeg, gif, svg, webp included
    - external_files -- db repr of a static file hosted somewhere
      - url
      - name
      - file_extension_id FK(file_extensions)
    - size_variants
      - variant_name -- e.g. full_size, thumbnail_64
    - images
      - external_file_id FK(external_files)
      - original_id FK(self)
      - size_variant_id FK(size_variant)
      - width
      - height
      - user_id
    - posts
      - thread_id FK(self) -- unites posts into a thread
      - parent_id FK(self) -- NULL is top-level post; w/value is comment
      - title
      - created_/updated_at -- updated via trigger
      - user_id
    - post_images -- 1-post:many-images
      - post_id
      - image_id
      - ordering
    - post_contents
      - post_id
      - plain_text
      - rich_text -- stores Quill.js Deltas
      
# notes on debugging

## handlers
- sometimes axum will complain if a type you're extracting doesn't implement Clone (Extension) or Deserialize(Json)

# mvp stories
as a dev, I want to ...
- run a command to generate a super user
- easily configure whether an email is printed or sent when developing locally

as a user, I want to ...
- receive an invite with a link to sign up
- click a link to visit the sign up page and input my username
- send an invite to a new prospective user
- enter my email or username on a log in page to receive a log in email
- click a link to get a cookie aka finish logging in
- old invites should expire -- should be checked when I visit the invites page

as an admin I want to ...
- view a list of users
- edit a specific user
- create a user
- delete a user

push
- users have limited number of invites
- invites have a status that can be viewed
- invites can be canceled