defmodule Memcached do
  alias Memcached.TcpServer

  @default_port 11211

  def start(port \\ @default_port) do
    TcpServer.listen(port)
  end
end
