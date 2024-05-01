defmodule JsonParser do
  defmodule Token do
    defstruct type: nil, children: []
  end

  def call(:eof), do: {:error, nil}

  def call(text) do
    inputs =
      text
      |> String.split("", trim: true)

    parse(inputs)
  end

  defp parse(inputs, result \\ nil)
  defp parse([], result), do: {:ok, result}

  defp parse(inputs, result) do
    {:ok, tail, result} = build_value(inputs, result)

    parse(tail, result)
  end

  defp build_value([], acc), do: {:ok, [], acc}

  defp build_value([char | tail], acc) do
    case char do
      "{" ->
        build_object(tail, %Token{type: :object})

      "\"" ->
        # Unsure if this is required; is "foo" considered valid JSON?
        build_string(tail, %Token{type: :string})

      _ ->
        build_value(tail, acc)
    end
  end

  defp build_object([char | tail], object) do
    case char do
      "}" ->
        {:ok, tail, object}

      "\"" ->
        {:ok, tail, member} = build_member(tail, %Token{type: :member})
        new_children = List.insert_at(object.children, -1, member)
        build_object(tail, %{object | children: new_children})

      _ ->
        build_object(tail, object)
    end
  end

  defp build_member(input, member) do
    {:ok, [name_sep | tail], string} = build_string(input, %Token{type: :string})

    case name_sep do
      ":" ->
        {:ok, tail, value} = build_value(tail, nil)

        {:ok, tail, %{member | children: member.children ++ [string, value]}}

      x ->
        raise ArgumentError, message: x
    end
  end

  defp build_string(["\"" | tail], string) do
    {:ok, tail, string}
  end

  defp build_string([char | tail], string) when is_binary(char) do
    build_string(tail, string)
  end
end

IO.read(:stdio, :eof)
|> JsonParser.call()
|> case do
  {:ok, result} ->
    IO.puts(:stderr, inspect(result))
    System.halt(0)

  {:error, _} ->
    System.halt(1)
end
