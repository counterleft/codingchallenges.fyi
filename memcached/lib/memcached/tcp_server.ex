defmodule Memcached.TcpServer do
  @moduledoc """
  The TcpServer module listens on the specified port for new client connections to our toy Memcached service.
  New clients are assigned their own Memcached.TcpWorker (as a GenServer).
  """

  def listen(port) do
    {:ok, socket} = :gen_tcp.listen(port, [:binary, packet: :line, active: true, reuseaddr: true])
    IO.puts("Accepting connection on port: #{port}")

    {:ok, agent_pid} = Memcached.RepoAgent.start_link()
    :sys.trace(agent_pid, true)

    accept_loop(socket)
  end

  def accept_loop(socket) do
    case :gen_tcp.accept(socket) do
      {:ok, client_socket} ->
        IO.puts("Client connected")
        {:ok, pid} = GenServer.start_link(Memcached.TcpWorker, socket)
        :gen_tcp.controlling_process(client_socket, pid)
        :sys.trace(pid, true)

      err ->
        IO.puts("#{err}")
    end

    accept_loop(socket)
  end

  # TODO Any :gen_tcp.shutdown() issues?
end
