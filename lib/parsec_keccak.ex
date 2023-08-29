defmodule ParsecKeccak do

  alias ParsecKeccak.Wrapper

  @default_opts [decode: false]

  def hash(string, opts \\ @default_opts) do
    case do_hash(string, opts) do
      :error -> {:error, "Hashing failed"}
      res    -> {:ok, res}
    end
  end

  # private

  defp do_hash(string, opts) when is_list(opts) do
    Wrapper.hash(string, Enum.into(opts, %{}))
  end
  defp do_hash(string, opts), do: Wrapper.hash(string, opts)
end
