defmodule Memcached.Repo do
  @moduledoc """
  The Repo represents the core logic for our toy Memcached service.
  It handles retrieval and storage of the given key-value data.
  """

  alias Memcached.Repo
  alias Memcached.Value

  defstruct storage: %{}

  @spec new() :: %Repo{}
  def new(existing_data \\ %{}) do
    %Repo{storage: existing_data}
  end

  @type get_cmd_resp ::
          {:ok, %Value{}} | {:error, :key_not_found} | {:error, :value_expired, %Repo{}}
  @spec get(%Repo{}, binary()) :: get_cmd_resp()
  def get(repo, key) do
    with {:ok, value} <- get_value(repo, key),
         {:ok, value} <- check_expiration(repo, key, value) do
      {:ok, value}
    end
  end

  defp check_expiration(_repo, _key, value) when value.exp_time == 0, do: {:ok, value}

  defp check_expiration(repo, key, value) do
    {:ok, now} = DateTime.now("Etc/UTC")
    seconds_diff = seconds_since(value.created_ts, now)

    case value.exp_time >= seconds_diff do
      true ->
        {:ok, value}

      false ->
        {_, updated_storage} = Map.pop(repo.storage, key)
        {:error, :value_expired, %Repo{storage: updated_storage}}
    end
  end

  defp get_value(repo, key) do
    case Map.get(repo.storage, key, :not_found) do
      :not_found ->
        {:error, :key_not_found}

      found ->
        {:ok, found}
    end
  end

  defp seconds_since(this_time, that_time) do
    {this_seconds, _} = DateTime.to_gregorian_seconds(this_time)
    {that_seconds, _} = DateTime.to_gregorian_seconds(that_time)

    abs(this_seconds - that_seconds)
  end

  def set(_repo, _key, %Value{flags: flags}) when flags < 0,
    do: {:error, :flags_out_of_bounds}

  def set(_repo, _key, %Value{flags: flags}) when flags > 65535,
    do: {:error, :flags_out_of_bounds}

  @spec set(%Repo{}, binary(), %Value{}) :: {:ok, %Repo{}}
  def set(repo, key, value) do
    repo = %Repo{
      storage: Map.put(repo.storage, key, value)
    }

    {:ok, repo}
  end
end
