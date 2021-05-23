=====================================
Example using app_data with actix-web
=====================================

This is a short example based on the `Shared Mutable State <https://actix.rs/docs/application/#shared-mutable-state>`_ example in the actix-web documentation.

I was running into an issue where I could not retrieve the AppStateWithCounter
struct after I moved the `index` handler out to a separate file, handlers.rs.

The issue turned out to be that using "mod state;" in the handlers.rs
causes the AppStateWithCounter to have a different ID than the one used in
main.rs. This caused a runtime error of "App data is not configured, to
configure use App::data()".

The solution (thanks folks on the discord channel), was to use to switch to
super::state::AppStateWithCounter. This allows the ID to be the same and
the retrieval to be successful.
