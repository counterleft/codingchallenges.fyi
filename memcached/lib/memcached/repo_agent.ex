defmodule Memcached.RepoAgent do
  @moduledoc """
  The RepoAgent manages the shared state for the key-value data.
  All processes that need to read/write to the Memcached service should use this module.
  """

  # TODO Maybe this Agent can be it's own GenServer so that get() and other "read"-commands can be non-blocking.
  # I say "read" because get() will do an expire-on-read semantic when expiry is involved.
  use Agent

  alias Memcached.Value
  alias Memcached.Repo

  def start_link do
    Agent.start_link(fn -> Repo.new() end, name: __MODULE__)
  end

  def set(key, value) do
    Agent.update(__MODULE__, fn state ->
      {:ok, new_state} = Repo.set(state, key, value)
      new_state
    end)
  end

  @spec get(binary()) :: {:ok, %Value{}} | {:error, :key_not_found} | {:error, :value_expired}
  def get(key) do
    Agent.get_and_update(__MODULE__, fn state ->
      case Repo.get(state, key) do
        {:ok, v} ->
          {{:ok, v}, state}

        {:error, :key_not_found} ->
          {{:error, :key_not_found}, state}

        {:error, :value_expired, updated_repo} ->
          {{:error, :value_expired}, updated_repo}
      end
    end)
  end
end
