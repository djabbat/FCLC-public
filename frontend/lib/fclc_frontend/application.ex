defmodule FCLCFrontend.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      FCLCFrontendWeb.Telemetry,
      {Phoenix.PubSub, name: FCLCFrontend.PubSub},
      FCLCFrontendWeb.Endpoint
    ]

    opts = [strategy: :one_for_one, name: FCLCFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    FCLCFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end