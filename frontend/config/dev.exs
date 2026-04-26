import Config

config :fclc_frontend, FCLCFrontendWeb.Endpoint,
  debug_errors: true,
  code_reloader: true,
  check_origin: false,
  watchers: [
    esbuild: {Esbuild, :install_and_run, [:fclc_frontend, ~w(--sourcemap=inline --watch)]},
    tailwind: {Tailwind, :install_and_run, [:fclc_frontend, ~w(--watch)]}
  ]

config :fclc_frontend, FCLCFrontendWeb.Endpoint,
  live_reload: [
    patterns: [
      ~r"priv/static/.*(js|css|png|jpeg|jpg|gif|svg)$",
      ~r"lib/fclc_frontend_web/(controllers|live|components)/.*(ex|heex)$"
    ]
  ]

config :fclc_frontend, FCLCFrontendWeb.BackendClient,
  backend_url: System.get_env("BACKEND_URL", "http://localhost:3001"),
  timeout: 60_000

config :logger, :console, format: "[$level] $message\n"
config :phoenix, :stacktrace_depth, 20
config :phoenix, :plug_init_mode, :runtime