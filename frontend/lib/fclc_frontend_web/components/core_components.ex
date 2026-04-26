defmodule FCLCFrontendWeb.CoreComponents do
  use Phoenix.Component

  alias Phoenix.LiveView.JS

  @doc """
  Renders a modal.

  ## Examples

      <.modal id="confirm-modal">
        This is a modal.
      </.modal>
  """
  attr :id, :string, required: true
  attr :show, :boolean, default: false
  attr :on_cancel, JS, default: %JS{}
  slot :inner_block, required: true

  def modal(assigns) do
    ~H"""
    <div
      id={@id}
      phx-mounted={@show && show_modal(@id)}
      phx-remove={hide_modal(@id)}
      class="relative z-50 hidden"
    >
      <div id={"#{@id}-bg"} class="fixed inset-0 bg-gray-500/90 transition-opacity" aria-hidden="true" />
      <div
        class="fixed inset-0 overflow-y-auto"
        aria-labelledby={"#{@id}-title"}
        aria-describedby={"#{@id}-description"}
        role="dialog"
        aria-modal="true"
        tabindex="0"
      >
        <div class="flex min-h-full items-center justify-center p-4">
          <div class="w-full max-w-2xl">
            <.focus_wrap
              id={"#{@id}-container"}
              phx-window-keydown={JS.exec("phx-remove", to: "##{@id}")}
              phx-key="escape"
              phx-click-away={JS.exec("phx-remove", to: "##{@id}")}
              class="hidden relative rounded-2xl bg-white p-6 shadow-xl shadow-zinc-700/10 ring-1 ring-zinc-700/10 transition"
            >
              <div class="absolute top-6 right-6">
                <button
                  phx-click={JS.push("close_modal", value: %{id: @id})}
                  type="button"
                  class="-m-3 flex-none p-3 opacity-20 hover:opacity-40"
                  aria-label="Close"
                >
                  ✕
                </button>
              </div>
              <div id={"#{@id}-content"}>
                <%= render_slot(@inner_block) %>
              </div>
            </.focus_wrap>
          </div>
        </div>
      </div>
    </div>
    """
  end

  @doc """
  Renders a loading spinner.
  """
  attr :class, :string, default: nil

  def spinner(assigns) do
    ~H"""
    <div class={["inline-block h-4 w-4 animate-spin rounded-full border-2 border-solid border-current border-r-transparent", @class]}>
    </div>
    """
  end

  @doc """
  Renders a badge with status coloring.
  """
  attr :status, :string, required: true
  attr :class, :string, default: nil
  slot :inner_block

  def status_badge(assigns) do
    colors = %{
      "online" => "bg-green-100 text-green-800",
      "offline" => "bg-red-100 text-red-800",
      "training" => "bg-blue-100 text-blue-800",
      "idle" => "bg-yellow-100 text-yellow-800",
      "error" => "bg-red-500 text-white",
      "completed" => "bg-green-500 text-white"
    }

    assigns = assign(assigns, :color_class, colors[@status] || "bg-gray-100 text-gray-800")

    ~H"""
    <span class={["inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium", @color_class, @class]}>
      <%= render_slot(@inner_block) %>
    </span>
    """
  end

  @doc """
  Renders a progress bar.
  """
  attr :value, :integer, required: true
  attr :max, :integer, default: 100
  attr :label, :string, default: nil
  attr :color, :string, default: "blue"

  def progress_bar(assigns) do
    percentage = Float.round(@value / @max * 100, 1)
    color_class = case @color do
      "red" -> "bg-red-600"
      "green" -> "bg-green-600"
      "yellow" -> "bg-yellow-600"
      "blue" -> "bg-blue-600"
      _ -> "bg-blue-600"
    end

    ~H"""
    <div class="w-full">
      <div class="flex justify-between mb-1">
        <span class="text-sm font-medium text-gray-700"><%= @label %></span>
        <span class="text-sm font-medium text-gray-700"><%= percentage %>%</span>
      </div>
      <div class="w-full bg-gray-200 rounded-full h-2.5">
        <div class={["h-2.5 rounded-full", color_class]} style={"width: #{percentage}%"}></div>
      </div>
    </div>
    """
  end

  @doc """
  Renders a data card.
  """
  attr :title, :string, required: true
  attr :value, :any, required: true
  attr :subtitle, :string, default: nil
  attr :trend, :string, default: nil
  attr :class, :string, default: nil

  def data_card(assigns) do
    ~H"""
    <div class={["bg-white overflow-hidden shadow rounded-lg", @class]}>
      <div class="px-4 py-5 sm:p-6">
        <dt class="text-sm font-medium text-gray-500 truncate"><%= @title %></dt>
        <dd class="mt-1 text-3xl font-semibold text-gray-900"><%= @value %></dd>
        <dd :if={@subtitle} class="mt-1 text-sm text-gray-500"><%= @subtitle %></dd>
        <dd :if={@trend} class="mt-1">
          <span class={if String.starts_with?(@trend, "+"), do: "text-green-600", else: "text-red-600"}>
            <%= @trend %>
          </span>
        </dd>
      </div>
    </div>
    """
  end

  defp show_modal(js \\ %JS{}, id) do
    js
    |> JS.show(to: "##{id}")
    |> JS.show(
      to: "##{id}-bg",
      transition: "transition-all duration-300 ease-out",
      time: 300
    )
    |> JS.show(
      to: "##{id}-container",
      transition: "transition-all duration-300 ease-out",
      time: 300
    )
    |> JS.add_class("overflow-hidden", to: "body")
  end

  defp hide_modal(js \\ %JS{}, id) do
    js
    |> JS.hide(
      to: "##{id}-bg",
      transition: "transition-all duration-300 ease-in",
      time: 300
    )
    |> JS.hide(
      to: "##{id}-container",
      transition: "transition-all duration-300 ease-in",
      time: 300
    )
    |> JS.hide(to: "##{id}", transition: "fade-out", time: 300)
    |> JS.remove_class("overflow-hidden", to: "body")
  end

  embed_templates "core_components/*"
end