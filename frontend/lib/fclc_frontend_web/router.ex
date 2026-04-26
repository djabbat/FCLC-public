defmodule FCLCFrontendWeb.Router do
  use FCLCFrontendWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {FCLCFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", FCLCFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive
    live "/nodes/:node_id", DetailLive, :node
    live "/rounds/:round_id", DetailLive, :round
    live "/contributions/:participant_id", DetailLive, :contribution
    live "/counters", DetailLive, :counter_registry
    live "/sensitivity", DetailLive, :sensitivity
    live "/lineage", DetailLive, :lineage
  end

  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser
      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end
end