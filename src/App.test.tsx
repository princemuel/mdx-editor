import App from "./App";
import { render, screen } from "./utils";

describe("App", () => {
  test("should render a heading one element", () => {
    render(<App />);
    const headingElement = screen.getByRole("heading", { level: 1 });
    expect(headingElement).toBeInTheDocument();
  });

  test("should render slogan", () => {
    render(<App />);
    const headingElement = screen.getByText(/vite \+ react/i);
    expect(headingElement).toBeInTheDocument();
  });
});
