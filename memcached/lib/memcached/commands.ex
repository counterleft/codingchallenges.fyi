defmodule Memcached.Commands do
  @moduledoc """
  The Commands module handles how the Memcached commands are sent to the Memcached.Repo (via Memcached.RepoAgent).
  It also handles the response formatting for each command.
  """

  alias Memcached.RepoAgent
  alias Memcached.Value

  def get(key) do
    case RepoAgent.get(key) do
      {:ok, value} ->
        [
          "VALUE #{key} #{value.flags} #{value.byte_count}",
          value.data,
          "END"
        ]
        |> List.flatten()

      {:error, :value_expired} ->
        ["END"]

      {:error, :key_not_found} ->
        ["END"]
    end
  end

  def set(key, flags, exp_time, byte_count, data) do
    {:ok, now} = DateTime.now("Etc/UTC")

    value = %Value{
      flags: String.to_integer(flags),
      byte_count: String.to_integer(byte_count),
      created_ts: now,
      exp_time: String.to_integer(exp_time),
      data: data
    }

    # TODO Errors?
    RepoAgent.set(key, value)

    ["STORED"]
  end
end
