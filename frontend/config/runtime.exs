import Config

if config_env() == :prod do
  host = System.get_env("HOST", "localhost")
  port = String.to_integer(System.get_env("PORT", "4001"))

  config :fclc_frontend, FCLCFrontendWeb.Endpoint,
    url: [host: host, port: port],
    http: [
      ip: {0, 0, 0, 0, 0, 0, 0, 0},
      port: port
    ],
    secret_key_base: System.fetch_env!("SECRET_KEY_BASE")

  config :fclc_frontend, FCLCFrontendWeb.BackendClient,
    backend_url: System.fetch_env!("BACKEND_URL")
end