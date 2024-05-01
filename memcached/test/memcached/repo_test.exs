defmodule Memcached.RepoTest do
  use ExUnit.Case

  alias Memcached.Repo, as: Repo

  describe "get command" do
    test "returns :error tuple when key doesn't exist" do
      repo = Repo.new()
      assert Repo.get(repo, "does not exist") == {:error, :key_not_found}
    end

    test "returns data and extras" do
      data = %{
        "k" => %Memcached.Value{
          data: "value",
          flags: 0,
          exp_time: 0,
          byte_count: 5,
          created_ts: DateTime.now!("Etc/UTC")
        }
      }

      repo = Repo.new(data)
      assert Repo.get(repo, "k") == {:ok, data["k"]}
    end

    test "exp_time: returns empty for expired value" do
      data = %{
        "k" => %Memcached.Value{
          data: "value",
          flags: 0,
          exp_time: 1,
          byte_count: 5,
          created_ts: DateTime.now!("Etc/UTC")
        }
      }

      repo = Repo.new(data)
      Process.sleep(2000)

      assert {:error, :value_expired, updated_repo} = Repo.get(repo, "k")
      assert Map.has_key?(updated_repo.storage, "k") == false
    end

    test "exp_time: 0 means no-expiration" do
      data = %{
        "k" => %Memcached.Value{
          data: "value",
          flags: 0,
          exp_time: 0,
          byte_count: 5,
          created_ts: DateTime.now!("Etc/UTC")
        }
      }

      repo = Repo.new(data)
      Process.sleep(2000)

      assert Repo.get(repo, "k") == {:ok, data["k"]}
    end
  end

  describe "set command" do
    test "successful set" do
      value = %Memcached.Value{
        data: "value",
        flags: 0,
        exp_time: 0,
        byte_count: 5,
        created_ts: DateTime.now!("Etc/UTC")
      }

      repo = Repo.new()
      expected_repo = Repo.new(%{"k" => value})

      assert Repo.set(repo, "k", value) == {:ok, expected_repo}
    end

    test "flags is an unsigned 16-bit integer" do
      repo = Repo.new()

      assert Repo.set(repo, "k", %Memcached.Value{
               flags: -1,
               data: "v",
               exp_time: 0,
               byte_count: 1,
               created_ts: DateTime.now!("Etc/UTC")
             }) == {:error, :flags_out_of_bounds}

      assert Repo.set(repo, "k", %Memcached.Value{
               flags: 65536,
               data: "v",
               exp_time: 0,
               byte_count: 1,
               created_ts: DateTime.now!("Etc/UTC")
             }) == {:error, :flags_out_of_bounds}
    end
  end
end
