/**
 * Greet should render the text hello and if a name is passed into the component
 * It should render hello followed by the name
 */
import { render, screen } from "@/utils";
import { Greet } from "./greet";

// .only or fit, .skip or xit, nestable, test or it, describe to group
describe("Greet", () => {
  test("should render correctly", () => {
    render(<Greet />);
    const textElement = screen.getByText(/Hey/);
    expect(textElement).toBeInTheDocument();
  });

  // describe("Nested", () => {
  //   it("should render a name", () => {
  //     render(<Greet name="Spencer" />);
  //     const textElement = screen.getByText("Hey Spencer");
  //     expect(textElement).toBeInTheDocument();
  //   });
  // });
});

// describe("Nested", () => {
//   it("should render a name", () => {
//     render(<Greet name="Spencer" />);
//     const textElement = screen.getByText("Hey Spencer");
//     expect(textElement).toBeInTheDocument();
//   });
// });
