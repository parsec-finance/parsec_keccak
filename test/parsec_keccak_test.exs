defmodule ParsecKeccakTest do
  use ExUnit.Case
  doctest ParsecKeccak

  test "hash" do
    tfer = "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"

    assert ParsecKeccak.hash("Transfer(address,address,uint256)") == tfer
  end
end
