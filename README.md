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

## Adding a New App
TODO: add a generator for this sort of thing?
- add the app in `apps/<app_name>`
- add `pub mod <app_name>` to `apps/mod.rs`
- IF there are HTML templates, add the path to the templates file to the `dirs` array in `askama.toml`. Note that this sucks and is the main reason I'm considering adding some sort of generator to this... 


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
    