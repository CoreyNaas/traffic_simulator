**Assignment: Integrate Pedestrian Crosswalks into the Traffic Light Simulator**

**Objective:**
To build upon the four-way intersection model from part one, this assignment introduces pedestrian crosswalks into the traffic light simulator. You are to integrate a pedestrian signaling system that interacts with the vehicular traffic light states, ensuring pedestrians can cross safely. This addition should be seamlessly incorporated into the existing state machine logic for the intersection.

**Requirements:**

1. **Pedestrian Signal Model:**
   Define a `PedestrianSignal` struct that represents the state of pedestrian walk/don't walk signals. This struct should include logic for timing pedestrian phases in relation to vehicular traffic lights.

2. **Enhanced Intersection Model:**
   Expand the `Intersection` struct to include `PedestrianSignal` instances, ensuring a logical association with the corresponding traffic lights.

3. **State Machine Logic Update:**
   Update the state machine to manage pedestrian signals in coordination with vehicular lights. Ensure that pedestrian signals change only when it is safe for them to do so. Introduce a delay between vehicular red lights and pedestrian walk signals to simulate buffer time for clearing vehicles.

4. **UX/UI Integration:**
   The user interface must now display the state of pedestrian signals in addition to vehicular traffic lights. The UI should clearly indicate when it is safe for pedestrians to cross and when they should wait.

5. **User Interaction Enhancement:**
   Provide UI controls for users to simulate the request of crossing as a pedestrian, testing the responsiveness of the system.

**Deliverables:**

1. **Code:**
   Submit the updated Rust source files with your implementation of pedestrian signaling, integrated within the `Intersection` logic.

2. **UI Demonstration:**
   A gif or video of the UI showing both the vehicular and pedestrian signals in operation.

3. **Documentation:**
   Update the README file to document your implementation of the pedestrian crosswalks, explaining how it was integrated with the existing vehicular state machine.

**Evaluation Criteria:**

- **Functionality:** The simulator must accurately model the operation of pedestrian signals in conjunction with vehicular traffic lights.
- **Code Quality:** The code should continue to maintain a clear separation of concerns, with logical integration between pedestrian and vehicular systems.
- **Usability:** The UI should be intuitive and reflect the current state of all signals in a way that would be clear to both drivers and pedestrians.

**Extra Credit Features:**

1. **SCADA-like Control Panel:**
   Develop a control panel in the UI akin to SCADA systems used for monitoring and controlling infrastructure. This panel should allow for manual overrides of signal states and provide a system status overview.

2. **Interactive Tutorial Mode:**
   Implement a tutorial mode within the UI that guides users through the operation of the intersection, including the new pedestrian signals. This mode should include interactive prompts and feedback to educate users on traffic light control systems.

**Hints:**
- Remember that pedestrians typically have a dedicated signal phase that does not conflict with vehicular traffic.
- Consider the impact of pedestrian phase timing on overall intersection efficiency and safety.

**Submission Deadline:**
[Insert deadline]

Upon completion, please submit your code repository link, along with the required deliverables, to [insert submission email or upload link].

This simulation will serve as a stepping stone towards creating a comprehensive SCADA for traffic light control systems. Your work will contribute to a deeper understanding of traffic control complexities and user experience design.

Good luck, and ensure your design prioritizes the safety of all intersection users!