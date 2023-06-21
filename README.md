# Rust Server Boilerplate
A template for starting new, lean, beautiful rust backends.

## Technologies
- Axum as the server framework with various Tower dependencies
- Askana for HTML templating
- SCSS for styling
- HTMX for front end reactivity
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

## Out of the Box
- A user app
  - views:
    - log_in
    - sign_up
  - handlers:
    - log_in
    - log_out
  - middleware
    - require_authentication
  - models
    - user model
      - username
      - email
      - active
      - created_at
      - updated_at
    

# notes on debugging

## handlers
- sometimes axum will complain if a type you're extracting doesn't implement Clone (Extension) or Deserialize(Json)