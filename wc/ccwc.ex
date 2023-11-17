defmodule CCWC do
  defmodule Inputs do
    defstruct [:files, :options]
  end

  defmodule Outputs do
    defstruct [:byte_count, :word_count, :line_count, :character_count, :input_filename]
  end

  defp get_text(input_filename) when input_filename == nil do
    # Then presume input is coming from stdin
    IO.read(:stdio, :eof)
  end

  defp get_text(input_filename) do
    File.read!(input_filename)
  end

  defp normalize_text(text) do
    text
    |> String.trim_trailing()
    |> String.replace(~r/\r\n|\r/, "\n")
  end

  def call(arg_list) do
    parsed_args = parse_arg_list!(arg_list)

    input_filename = Enum.at(parsed_args.files, 0)
    text = get_text(input_filename)
    # IO.puts(text)

    output =
      case parsed_args.options do
        [] ->
          %Outputs{
            line_count: do_line_count(text),
            word_count: do_word_count(text),
            byte_count: do_byte_count(text),
            input_filename: input_filename
          }

        [lines: true] ->
          %Outputs{
            line_count: do_line_count(text),
            input_filename: input_filename
          }

        [words: true] ->
          %Outputs{
            word_count: do_word_count(text),
            input_filename: input_filename
          }

        [bytes: true] ->
          %Outputs{
            byte_count: do_byte_count(text),
            input_filename: input_filename
          }

        [characters: true] ->
          %Outputs{
            character_count: do_character_count(text),
            input_filename: input_filename
          }
      end

    format_output(output)
  end

  defp parse_arg_list!(arg_list) do
    {options, args} =
      OptionParser.parse!(arg_list,
        aliases: [c: :bytes, l: :lines, w: :words, m: :characters],
        strict: [bytes: :boolean, lines: :boolean, words: :boolean, characters: :boolean]
      )

    %Inputs{options: options, files: args}
  end

  defp format_output(outputs) do
    # -l -w -c input_filename
    byte_count = pad_leading(outputs.byte_count)
    line_count = pad_leading(outputs.line_count)
    word_count = pad_leading(outputs.word_count)
    character_count = pad_leading(outputs.character_count)

    counts =
      [line_count, word_count, byte_count, character_count]
      |> Enum.reject(fn count -> count == nil end)
      |> Enum.join()

    case outputs.input_filename do
      nil ->
        counts

      input_filename ->
        counts <> " #{input_filename}"
    end
  end

  defp pad_leading(nil), do: nil

  defp pad_leading(value) do
    value
    |> to_string()
    |> String.pad_leading(8)
  end

  defp do_byte_count(text) do
    byte_size(text)
  end

  defp do_line_count(text) do
    # text
    # |> String.split("\n")
    # |> length()
    text
    |> normalize_text()
    |> String.split("")
    |> Enum.count(fn char -> char == "\n" end)
  end

  defp do_word_count(text) do
    text
    |> String.split([" ", "\n", "\t", "\f", "\v", "\r"], trim: true)
    |> length()
    |> to_string()
  end

  defp do_character_count(text) do
    text
    |> String.codepoints()
    |> length()
  end
end

System.argv()
|> CCWC.call()
|> IO.puts()
