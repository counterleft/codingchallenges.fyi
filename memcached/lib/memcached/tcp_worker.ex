defmodule Memcached.TcpWorker do
  @moduledoc """
  The TcpWorker module keeps the TCP connection alive for a given client.
  It collects all the input sent from the client over the network and 
  forwards it to the Command module to do the actual work with the Memcached service.
  """
  use GenServer

  def init(socket) do
    {:ok, socket}
  end

  def handle_info({:tcp, socket, <<"set", args::binary>>}, _state) do
    # TODO Invalid pattern; invalid set command
    [key, flags, exp_time, byte_count] =
      args
      |> String.trim()
      |> String.split(" ", trim: true)

    {:noreply,
     {socket, ["set", key, flags, exp_time, byte_count], String.to_integer(byte_count), []}}
  end

  def handle_info(
        {:tcp, socket, data},
        {socket, ["set", key, flags, exp_time, byte_count] = cmd, remaining_bytes, current_data}
      ) do
    parsed =
      data
      |> String.trim()
      |> String.slice(0..(remaining_bytes - 1))

    remaining_bytes = remaining_bytes - String.length(parsed)

    case remaining_bytes <= 0 do
      true ->
        Memcached.Commands.set(key, flags, exp_time, byte_count, current_data ++ [parsed])
        |> write_responses(socket)

        {:noreply, socket}

      false ->
        {:noreply, {socket, cmd, remaining_bytes, current_data ++ [parsed]}}
    end
  end

  def handle_info({:tcp, socket, <<"get", args::binary>>}, _state) do
    key =
      args
      |> String.trim()
      |> String.split(" ", trim: true)
      |> List.first()

    Memcached.Commands.get(key)
    |> write_responses(socket)

    {:noreply, socket}
  end

  def handle_info({:tcp_closed, _socket}, state) do
    IO.puts("Socket closed")
    {:noreply, state}
  end

  def handle_info({:tcp_error, _socket, reason}, state) do
    IO.puts("Error: #{reason}")
    {:noreply, state}
  end

  defp write_responses(responses, socket) do
    Enum.each(responses, fn r ->
      :gen_tcp.send(socket, r <> "\n")
    end)
  end
end
