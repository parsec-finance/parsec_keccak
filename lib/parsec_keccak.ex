defmodule ParsecKeccak do
use Rustler,
  otp_app: :parsec_keccak,
  crate: :parseckeccak

  def hash(_arg1), do: :erlang.nif_error(:nif_not_loaded)
end
