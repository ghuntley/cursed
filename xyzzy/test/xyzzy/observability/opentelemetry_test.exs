defmodule Xyzzy.Observability.OpenTelemetryTest do
  use ExUnit.Case

  alias Xyzzy.Observability.OpenTelemetry

  describe "OpenTelemetry.setup/0" do
    test "setup function doesn't crash" do
      assert OpenTelemetry.setup() == :ok
    end
  end
end