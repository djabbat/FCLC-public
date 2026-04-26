import Config

config :fclc_frontend, FCLCFrontendWeb.Endpoint,
  url: [host: System.get_env("HOST", "localhost"), port: 4001],
  cache_static_manifest: "priv/static/cache_manifest.json"

config :fclc_frontend, FCLCFrontendWeb.BackendClient,
  backend_url: System.get_env("BACKEND_URL"),
  timeout: 45_000

config :logger,
  level: :info,
  truncate: :infinity

config :phoenix, :serve_endpoints, true