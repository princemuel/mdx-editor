import App from "./App";
import { render, screen } from "./utils";

test("renders slogan", () => {
  render(<App />);
  const headingElement = screen.getByText(/vite \+ react/i);
  expect(headingElement).toBeInTheDocument();
});
