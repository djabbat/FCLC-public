defmodule FCLCFrontendWeb.DetailLive do
  use FCLCFrontendWeb, :live_view

  alias FCLCFrontendWeb.BackendClient
  alias FCLCFrontendWeb.CoreComponents

  @impl true
  def mount(_params, _session, socket) do
    {:ok, assign(socket, :loading, true)}
  end

  @impl true
  def handle_params(%{"node_id" => id}, _uri, socket) do
    send(self(), {:fetch_node, id})
    {:noreply, assign(socket, :entity_type, :node)}
  end

  def handle_params(%{"round_id" => id}, _uri, socket) do
    send(self(), {:fetch_round, id})
    {:noreply, assign(socket, :entity_type, :round)}
  end

  def handle_params(%{"participant_id" => id}, _uri, socket) do
    send(self(), {:fetch_contribution, id})
    {:noreply, assign(socket, :entity_type, :contribution)}
  end

  def handle_params(%{"counter_registry" => _}, _uri, socket) do
    send(self(), {:fetch_counters})
    {:noreply,
     assign(socket, :entity_type, :counter_registry)
     |> assign(:counters, [])}
  end

  def handle_params(%{"sensitivity" => _}, _uri, socket) do
    send(self(), {:fetch_sensitivity})
    {:noreply,
     assign(socket, :entity_type, :sensitivity)
     |> assign(:sensitivity_data, [])}
  end

  def handle_params(%{"lineage" => _}, _uri, socket) do
    send(self(), {:fetch_lineage})
    {:noreply,
     assign(socket, :entity_type, :lineage)
     |> assign(:lineage_data, [])}
  end

  def handle_params(_params, _uri, socket) do
    {:noreply, push_navigate(socket, to: "/")}
  end

  @impl true
  def handle_info({:fetch_node, id}, socket) do
    case BackendClient.get_node(id) do
      {:ok, node} ->
        {:noreply,
         socket
         |> assign(:entity, node)
         |> assign(:loading, false)}

      {:error, error} ->
        {:noreply,
         socket
         |> put_flash(:error, "Failed to load node: #{error}")
         |> push_navigate(to: "/")
         |> assign(:loading, false)}
    end
  end

  def handle_info({:fetch_round, id}, socket) do
    case BackendClient.get_round(id) do
      {:ok, round} ->
        {:noreply,
         socket
         |> assign(:entity, round)
         |> assign(:loading, false)}

      {:error, error} ->
        {:noreply,
         socket
         |> put_flash(:error, "Failed to load round: #{error}")
         |> push_navigate(to: "/")
         |> assign(:loading, false)}
    end
  end

  def handle_info({:fetch_contribution, id}, socket) do
    case BackendClient.get_contribution(id) do
      {:ok, contribution} ->
        {:noreply,
         socket
         |> assign(:entity, contribution)
         |> assign(:loading, false)}

      {:error, error} ->
        {:noreply,
         socket
         |> put_flash(:error, "Failed to load contribution: #{error}")
         |> push_navigate(to: "/")
         |> assign(:loading, false)}
    end
  end

  def handle_info({:fetch_counters}, socket) do
    case BackendClient.get_counters() do
      {:ok, counters} ->
        {:noreply,
         socket
         |> assign(:counters, counters)
         |> assign(:loading, false)}

      {:error, error} ->
        {:noreply,
         socket
         |> assign(:counters, [])
         |> assign(:loading, false)
         |> put_flash(:error, "Failed to load counters: #{error}")}
    end
  end

  def handle_info({:fetch_sensitivity}, socket) do
    case BackendClient.get_sensitivity() do
      {:ok, data} ->
        {:noreply,
         socket
         |> assign(:sensitivity_data, data)
         |> assign(:loading, false)}

      {:error, error} ->
        {:noreply,
         socket
         |> assign(:sensitivity_data, [])
         |> assign(:loading, false)
         |> put_flash(:error, "Failed to load sensitivity data: #{error}")}
    end
  end

  def handle_info({:fetch_lineage}, socket) do
    case BackendClient.get_lineage() do
      {:ok, data} ->
        {:noreply,
         socket
         |> assign(:lineage_data, data)
         |> assign(:loading, false)}

      {:error, error} ->
        {:noreply,
         socket
         |> assign(:lineage_data, [])
         |> assign(:loading, false)
         |> put_flash(:error, "Failed to load lineage data: #{error}")}
    end
  end

  def render(assigns) do
    case assigns.entity_type do
      :node -> render_node(assigns)
      :round -> render_round(assigns)
      :contribution -> render_contribution(assigns)
      :counter_registry -> render_counter_registry(assigns)
      :sensitivity -> render_sensitivity(assigns)
      :lineage -> render_lineage(assigns)
    end
  end

  defp render_node(assigns) do
    ~H"""
    <div class="py-6">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <.back_button />
        
        <%= if @loading do %>
          <div class="flex justify-center items-center h-64">
            <CoreComponents.spinner class="h-8 w-8" />
            <span class="ml-2">Loading node details...</span>
          </div>
        <% else %>
          <div class="bg-white shadow overflow-hidden sm:rounded-lg">
            <div class="px-4 py-5 sm:px-6">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="text-lg leading-6 font-medium text-gray-900">
                    <%= @entity.name %>
                  </h3>
                  <p class="mt-1 max-w-2xl text-sm text-gray-500">
                    <%= @entity.institution %> • Node ID: <%= @entity.id %>
                  </p>
                </div>
                <CoreComponents.status_badge status={@entity.status}>
                  <%= @entity.status %>
                </CoreComponents.status_badge>
              </div>
            </div>
            <div class="border-t border-gray-200">
              <dl>
                <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Records Count</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= @entity.records_count %>
                  </dd>
                </div>
                <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Ze Value</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    v = Nₛ/(N-1) = <%= @entity.ze_value %>
                  </dd>
                </div>
                <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Contribution Score</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <CoreComponents.progress_bar
                      value={@entity.contribution_score}
                      max={100}
                      label="Shapley value"
                      color="blue"
                    />
                  </dd>
                </div>
                <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Last Training Round</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= @entity.last_round || "N/A" %>
                  </dd>
                </div>
                <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Data Domains</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= Enum.join(@entity.domains || [], ", ") %>
                  </dd>
                </div>
                <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Privacy Budget Used</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    ε = <%= @entity.epsilon_used %> / ≤1.0
                  </dd>
                </div>
              </dl>
            </div>
          </div>
        <% end %>
      </div>
    </div>
    """
  end

  defp render_round(assigns) do
    ~H"""
    <div class="py-6">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <.back_button />
        
        <%= if @loading do %>
          <div class="flex justify-center items-center h-64">
            <CoreComponents.spinner class="h-8 w-8" />
            <span class="ml-2">Loading round details...</span>
          </div>
        <% else %>
          <div class="bg-white shadow overflow-hidden sm:rounded-lg">
            <div class="px-4 py-5 sm:px-6">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="text-lg leading-6 font-medium text-gray-900">
                    Training Round <%= @entity.round_number %>
                  </h3>
                  <p class="mt-1 max-w-2xl text-sm text-gray-500">
                    SecAgg+ • Federated Averaging • <%= @entity.timestamp %>
                  </p>
                </div>
                <div class="flex space-x-2">
                  <CoreComponents.status_badge status={@entity.status}>
                    <%= @entity.status %>
                  </CoreComponents.status_badge>
                  <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800">
                    ε=<%= @entity.epsilon %>
                  </span>
                </div>
              </div>
            </div>
            <div class="border-t border-gray-200">
              <dl>
                <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Participants</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= @entity.participants_count %> nodes
                  </dd>
                </div>
                <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Model Accuracy</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= @entity.accuracy %>%
                  </dd>
                </div>
                <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Loss</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= Float.round(@entity.loss, 4) %>
                  </dd>
                </div>
                <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Aggregation Method</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= @entity.aggregation_method %>
                  </dd>
                </div>
                <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Dropout Recovery</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= @entity.dropout_recovery ? "Yes (Shamir t,n)" : "No" %>
                  </dd>
                </div>
                <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Gradient Norm</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    L2 ≤ <%= @entity.gradient_norm %>
                  </dd>
                </div>
              </dl>
            </div>
          </div>
        <% end %>
      </div>
    </div>
    """
  end

  defp render_contribution(assigns) do
    ~H"""
    <div class="py-6">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <.back_button />
        
        <%= if @loading do %>
          <div class="flex justify-center items-center h-64">
            <CoreComponents.spinner class="h-8 w-8" />
            <span class="ml-2">Loading contribution details...</span>
          </div>
        <% else %>
          <div class="bg-white shadow overflow-hidden sm:rounded-lg">
            <div class="px-4 py-5 sm:px-6">
              <h3 class="text-lg leading-6 font-medium text-gray-900">
                Contribution Analysis: <%= @entity.participant_name %>
              </h3>
              <p class="mt-1 max-w-2xl text-sm text-gray-500">
                Monte Carlo Shapley Value Estimation • <%= @entity.institution %>
              </p>
            </div>
            <div class="border-t border-gray-200">
              <dl>
                <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Shapley Value</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= Float.round(@entity.shapley_value, 4) %>
                  </dd>
                </div>
                <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Contribution Credits</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= @entity.credits %>
                  </dd>
                </div>
                <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Rank</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= @entity.rank %> / <%= @entity.total_participants %>
                  </dd>
                </div>
                <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Monte Carlo Iterations</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    <%= @entity.monte_carlo_iterations %>
                  </dd>
                </div>
                <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                  <dt class="text-sm font-medium text-gray-500">Marginal Impact</dt>
                  <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                    +<%= Float.round(@entity.marginal_impact * 100, 2) %>% accuracy improvement
                  </dd>
                </div>
              </dl>
            </div>
          </div>
        <% end %>
      </div>
    </div>
    """
  end

  defp render_counter_registry(assigns) do
    ~H"""
    <div class="py-6">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <.back_button />
        <h2 class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl mb-6">
          MCOA Counter Registry
        </h2>
        
        <%= if @loading do %>
          <div class="flex justify-center items-center h-64">
            <CoreComponents.spinner class="h-8 w-8" />
            <span class="ml-2">Loading counter registry...</span>
          </div>
        <% else %>
          <div class="bg-white shadow overflow-hidden sm:rounded-lg">
            <div class="px-4 py-5 sm:px-6">
              <h3 class="text-lg leading-6 font-medium text-gray-900">
                Registered Counters
              </h3>
              <p class="mt-1 max-w-2xl text-sm text-gray-500">
                Tissue-weighted calibration counters for MCOA framework
              </p>
            </div>
            <div class="border-t border-gray-200">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                  <tr>
                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Counter ID
                    </th>
                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Tissue Type
                    </th>
                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Weight (w_i)
                    </th>
                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Validation Status
                    </th>
                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Registered Nodes
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <%= for counter <- @counters do %>
                    <tr>
                      <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                        <%= counter.id %>
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        <%= counter.tissue_type %>
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        <%= Float.round(counter.weight, 4) %>
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap">
                        <CoreComponents.status_badge status={counter.validation_status}>
                          <%= counter.validation_status %>
                        </CoreComponents.status_badge>
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        <%= counter.registered_nodes %>
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

  defp render_sensitivity(assigns) do
    ~H"""