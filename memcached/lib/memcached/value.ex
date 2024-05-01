defmodule Memcached.Value do
  @enforce_keys [:flags, :byte_count, :exp_time, :created_ts]
  defstruct [:data, :flags, :byte_count, :exp_time, :created_ts]
end
