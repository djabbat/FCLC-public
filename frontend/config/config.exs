import Config

config :fclc_frontend,
  ecto_repos: [],
  generators: [timestamp_type: :utc_datetime]

config :fclc_frontend, FCLCFrontendWeb.Endpoint,
  url: [host: "localhost"],
  render_errors: [
    formats: [html: FCLCFrontendWeb.ErrorHTML, json: FCLCFrontendWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: FCLCFrontend.PubSub,
  live_view: [signing_salt: "abcdefgh"]

config :fclc_frontend, FCLCFrontendWeb.BackendClient,
  backend_url: System.get_env("BACKEND_URL", "http://localhost:3001"),
  timeout: 30_000,
  retry_attempts: 3

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason

import_config "#{config_env()}.exs"