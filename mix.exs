defmodule ParsecKeccak.MixProject do
  use Mix.Project

  def project() do
    [
      app: :parsec_keccak,
      version: "0.1.0",
      elixir: "~> 1.11",
      start_permanent: Mix.env() == :prod,
      description: description(),
      package: package(),
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application() do
    [
      extra_applications: [:logger]
    ]
  end

  # private

  defp description() do
    "Rust extension for string -> keccak hash"
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps() do
    [
      {:rustler, "~> 0.29.1"},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false},
    ]
  end

  defp package() do
    [
      name: "parsec_keccak",
      files: ~w(
        lib
        native
        priv
        test
        README*
        mix*
      ),
      licenses: ["Apache-2.0"],
      links: %{"GitHub" => "https://github.com/parsec-finance/parsec_keccak"}
    ]
  end
end
