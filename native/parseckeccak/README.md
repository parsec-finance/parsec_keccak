# NIF for Elixir.ParsecKeccak

## To build the NIF module:

- Your NIF will now build along with your project.

## To load the NIF:

```elixir
defmodule ParsecKeccak do
  use Rustler, otp_app: :parsec_keccak, crate: "parseckeccak"

  # When your NIF is loaded, it will override this function.
  def hash(_arg1), do: :erlang.nif_error(:nif_not_loaded)
end
```

## Examples

[This](https://github.com/rusterlium/NifIo) is a complete example of a NIF written in Rust.
