defmodule FCLCFrontend.MixProject do
  use Mix.Project

  def project do
    [
      app: :fclc_frontend,
      version: "0.1.0",
      elixir: "~> 1.14",
      elixirc_paths: elixirc_paths(Mix.env()),
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps(),
      dialyzer: [
        plt_file: {:no_warn, "priv/plts/dialyzer.plt"}
      ]
    ]
  end

  def application do
    [
      mod: {FCLCFrontend.Application, []},
      extra_applications: [:logger, :runtime_tools, :os_mon]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp deps do
    [
      # Phoenix
      {:phoenix, "~> 1.7.2"},
      {:phoenix_live_view, "~> 0.18.18"},
      {:phoenix_html, "~> 3.3"},
      {:phoenix_live_reload, "~> 1.2", only: :dev},
      {:phoenix_live_dashboard, "~> 0.7.2"},
      {:telemetry_metrics, "~> 0.6"},
      {:telemetry_poller, "~> 1.0"},
      
      # HTTP Client
      {:req, "~> 0.3.0"},
      
      # JSON
      {:jason, "~> 1.4"},
      
      # Runtime
      {:plug_cowboy, "~> 2.5"},
      
      # Development & Testing
      {:esbuild, "~> 0.7", runtime: Mix.env() == :dev},
      {:tailwind, "~> 0.2.0", runtime: Mix.env() == :dev},
      {:floki, ">= 0.30.0", only: :test},
      {:dialyxir, "~> 1.3", only: [:dev], runtime: false}
    ]
  end

  defp aliases do
    [
      setup: ["deps.get", "assets.setup", "assets.build"],
      "assets.setup": ["tailwind.install --if-missing", "esbuild.install --if-missing"],
      "assets.build": ["tailwind fclc_frontend", "esbuild fclc_frontend"],
      "assets.deploy": ["tailwind fclc_frontend --minify", "esbuild fclc_frontend --minify", "phx.digest"]
    ]
  end
end