defmodule FCLCFrontendWeb.DashboardLive do
  use FCLCFrontendWeb, :live_view

  alias FCLCFrontendWeb.BackendClient
  alias FCLCFrontendWeb.CoreComponents

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket), do: schedule_refresh()

    {:ok,
     socket
     |> assign(:loading, true)
     |> assign(:nodes, [])
     |> assign(:rounds, [])
     |> assign(:contributions, [])
     |> assign(:system_status, %{})
     |> assign(:error, nil), temporary_assigns: [nodes: [], rounds: [], contributions: []]}
  end

  @impl true
  def handle_params(_params, _uri, socket) do
    {:noreply, fetch_data(socket)}
  end

  @impl true
  def handle_event("refresh", _, socket) do
    {:noreply, fetch_data(socket)}
  end

  @impl true
  def handle_info(:refresh, socket) do
    schedule_refresh()
    {:noreply, fetch_data(socket)}
  end

  @impl true
  def handle_info({:backend_error, error}, socket) do
    {:noreply,
     socket
     |> assign(:error, "Backend error: #{error}")
     |> assign(:loading, false)}
  end

  def render(assigns) do
    ~H"""
    <div class="py-6">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="lg:flex lg:items-center lg:justify-between">
          <div class="min-w-0 flex-1">
            <h2 class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl">
              FCLC Dashboard
            </h2>
            <div class="mt-1 flex flex-col sm:mt-0 sm:flex-row sm:flex-wrap sm:space-x-6">
              <div class="mt-2 flex items-center text-sm text-gray-500">
                Federated Clinical Learning Cooperative v6.2
              </div>
              <div class="mt-2 flex items-center text-sm text-gray-500">
                SecAgg+ • ε≤1.0 • Monte Carlo Shapley
              </div>
            </div>
          </div>
          <div class="mt-5 flex lg:mt-0 lg:ml-4">
            <button
              phx-click="refresh"
              class="ml-3 inline-flex items-center rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
            >
              Refresh
            </button>
          </div>
        </div>
      </div>

      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 mt-8">
        <.flash_group flash={@flash} />
        
        <%= if @error do %>
          <div class="rounded-md bg-red-50 p-4 mb-6">
            <div class="flex">
              <div class="ml-3">
                <h3 class="text-sm font-medium text-red-800">Error</h3>
                <div class="mt-2 text-sm text-red-700">
                  <p><%= @error %></p>
                </div>
              </div>
            </div>
          </div>
        <% end %>

        <%= if @loading do %>
          <div class="flex justify-center items-center h-64">
            <CoreComponents.spinner class="h-8 w-8" />
            <span class="ml-2">Loading dashboard data...</span>
          </div>
        <% else %>
          <!-- System Status -->
          <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4 mb-8">
            <CoreComponents.data_card
              title="Active Nodes"
              value={length(@nodes)}
              subtitle="Federated participants"
            />
            <CoreComponents.data_card
              title="Training Rounds"
              value={length(@rounds)}
              subtitle="FedAvg + FedProx"
            />
            <CoreComponents.data_card
              title="Privacy Budget ε"
              value={@system_status[:epsilon] || "≤1.0"}
              subtitle="Differential Privacy"
              trend="-0.2"
            />
            <CoreComponents.data_card
              title="Contribution Credits"
              value={@system_status[:total_credits] || "0"}
              subtitle="Shapley value distribution"
            />
          </div>

          <!-- Nodes Grid -->
          <div class="mb-8">
            <h3 class="text-lg font-medium text-gray-900 mb-4">Federated Nodes</h3>
            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
              <%= for node <- @nodes do %>
                <div class="bg-white overflow-hidden shadow rounded-lg">
                  <div class="px-4 py-5 sm:p-6">
                    <div class="flex items-center">
                      <div class="flex-shrink-0">
                        <CoreComponents.status_badge status={node.status}>
                          <%= node.status %>
                        </CoreComponents.status_badge>
                      </div>
                      <div class="ml-3">
                        <h4 class="text-sm font-medium text-gray-900"><%= node.name %></h4>
                        <p class="text-sm text-gray-500"><%= node.institution %></p>
                      </div>
                    </div>
                    <div class="mt-4">
                      <CoreComponents.progress_bar
                        value={node.contribution_score || 0}
                        max={100}
                        label="Contribution Score"
                        color="blue"
                      />
                    </div>
                    <div class="mt-4 grid grid-cols-2 gap-4">
                      <div>
                        <p class="text-xs text-gray-500">Records</p>
                        <p class="text-sm font-medium"><%= node.records_count || 0 %></p>
                      </div>
                      <div>
                        <p class="text-xs text-gray-500">v = Nₛ/(N-1)</p>
                        <p class="text-sm font-medium"><%= node.ze_value || "0.0" %></p>
                      </div>
                    </div>
                    <div class="mt-4">
                      <a
                        href={"/nodes/#{node.id}"}
                        class="text-sm font-medium text-blue-600 hover:text-blue-500"
                      >
                        View details →
                      </a>
                    </div>
                  </div>
                </div>
              <% end %>
            </div>
          </div>

          <!-- Recent Rounds -->
          <div class="mb-8">
            <h3 class="text-lg font-medium text-gray-900 mb-4">Recent Training Rounds</h3>
            <div class="bg-white shadow overflow-hidden sm:rounded-md">
              <ul role="list" class="divide-y divide-gray-200">
                <%= for round <- @rounds do %>
                  <li>
                    <a href={"/rounds/#{round.id}"} class="block hover:bg-gray-50">
                      <div class="px-4 py-4 sm:px-6">
                        <div class="flex items-center justify-between">
                          <div class="flex items-center">
                            <CoreComponents.status_badge status={round.status}>
                              <%= round.status %>
                            </CoreComponents.status_badge>
                            <p class="ml-3 text-sm font-medium text-gray-900">
                              Round <%= round.round_number %> • SecAgg+
                            </p>
                          </div>
                          <div class="ml-2 flex-shrink-0 flex">
                            <p class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                              ε=<%= round.epsilon %>
                            </p>
                          </div>
                        </div>
                        <div class="mt-2 sm:flex sm:justify-between">
                          <div class="sm:flex">
                            <p class="flex items-center text-sm text-gray-500">
                              Participants: <%= round.participants_count %>
                            </p>
                          </div>
                          <div class="mt-2 flex items-center text-sm text-gray-500 sm:mt-0">
                            <p>
                              Accuracy: <%= round.accuracy %>% • Loss: <%= Float.round(round.loss, 4) %>
                            </p>
                          </div>
                        </div>
                      </div>
                    </a>
                  </li>
                <% end %>
              </ul>
            </div>
          </div>

          <!-- Top Contributors -->
          <div>
            <h3 class="text-lg font-medium text-gray-900 mb-4">Top Contributors (Shapley Values)</h3>
            <div class="bg-white shadow overflow-hidden sm:rounded-lg">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                  <tr>
                    <th
                      scope="col"
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Participant
                    </th>
                    <th
                      scope="col"
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Institution
                    </th>
                    <th
                      scope="col"
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Contribution Score
                    </th>
                    <th
                      scope="col"
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Credits
                    </th>
                    <th scope="col" class="relative px-6 py-3">
                      <span class="sr-only">View</span>
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <%= for contribution <- @contributions do %>
                    <tr>
                      <td class="px-6 py-4 whitespace-nowrap">
                        <div class="flex items-center">
                          <div class="ml-4">
                            <div class="text-sm font-medium text-gray-900">
                              <%= contribution.participant_name %>
                            </div>
                          </div>
                        </div>
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap">
                        <div class="text-sm text-gray-900"><%= contribution.institution %></div>
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap">
                        <CoreComponents.progress_bar
                          value={contribution.score}
                          max={100}
                          label=""
                          color="green"
                        />
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        <%= contribution.credits %>
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                        <a
                          href={"/contributions/#{contribution.participant_id}"}
                          class="text-blue-600 hover:text-blue-900"
                        >
                          Details
                        </a>
                      </td>
                    </tr>
                  <% end %>
                </tbody>
              </table>
            </div>
          </div>
        <% end %>
      </div>
    </div>
    """
  end

  defp fetch_data(socket) do
    socket = assign(socket, :loading, true)
    send(self(), {:fetch_nodes, 1})
    send(self(), {:fetch_rounds, 1})
    send(self(), {:fetch_contributions, 1})
    send(self(), {:fetch_system_status, 1})
    socket
  end

  defp handle_info({:fetch_nodes, attempt}, socket) do
    case BackendClient.list_nodes() do
      {:ok, nodes} ->
        {:noreply, assign(socket, :nodes, nodes)}

      {:error, _} when attempt < 3 ->
        Process.send_after(self(), {:fetch_nodes, attempt + 1}, 1000)
        {:noreply, socket}

      {:error, error} ->
        send(self(), {:backend_error, "Failed to fetch nodes: #{inspect(error)}"})
        {:noreply, assign(socket, :nodes, [])}
    end
  end

  defp handle_info({:fetch_rounds, attempt}, socket) do
    case BackendClient.list_rounds() do
      {:ok, rounds} ->
        {:noreply, assign(socket, :rounds, rounds)}

      {:error, _} when attempt < 3 ->
        Process.send_after(self(), {:fetch_rounds, attempt + 1}, 1000)
        {:noreply, socket}

      {:error, error} ->
        send(self(), {:backend_error, "Failed to fetch rounds: #{inspect(error)}"})
        {:noreply, assign(socket, :rounds, [])}
    end
  end

  defp handle_info({:fetch_contributions, attempt}, socket) do
    case BackendClient.list_contributions() do
      {:ok, contributions} ->
        {:noreply, assign(socket, :contributions, contributions)}

      {:error, _} when attempt < 3 ->
        Process.send_after(self(), {:fetch_contributions, attempt + 1}, 1000)
        {:noreply, socket}

      {:error, error} ->
        send(self(), {:backend_error, "Failed to fetch contributions: #{inspect(error)}"})
        {:noreply, assign(socket, :contributions, [])}
    end
  end

  defp handle_info({:fetch_system_status, attempt}, socket) do
    case BackendClient.get_system_status() do
      {:ok, status} ->
        {:noreply,
         socket
         |> assign(:system_status, status)
         |> assign(:loading, false)}

      {:error, _} when attempt < 3 ->
        Process.send_after(self(), {:fetch_system_status, attempt + 1}, 1000)
        {:noreply, socket}

      {:error, error} ->
        send(self(), {:backend_error, "Failed to fetch system status: #{inspect(error)}"})
        {:noreply,
         socket
         |> assign(:system_status, %{})
         |> assign(:loading, false)}
    end
  end

  defp schedule_refresh do
    Process.send_after(self(), :refresh, 30000)
  end
end