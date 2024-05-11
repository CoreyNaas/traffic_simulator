**Assignment: Develop a Data Analytics Module for the Traffic Light Simulator with Simulated Traffic Flow**

**Objective:**
The final part of this assignment series is to develop a comprehensive Data Analytics Module within the traffic light simulator to assess and optimize traffic flow. This module will simulate complex traffic patterns and provide insights into the efficiency and effectiveness of the implemented traffic control strategies.

**Requirements:**

1. **Traffic Flow Simulation:**
   Design a simulation engine capable of generating vehicles with varying characteristics (e.g., arrival rates, turn probabilities) and pedestrian traffic patterns. This engine will feed into the turn lanes, straight lanes, and pedestrian crossings of the intersection.

2. **Data Collection Infrastructure:**
   Implement a system within the `Intersection` that collects detailed data on traffic light states, vehicle and pedestrian wait times, and lane usage statistics.

3. **Analytics Dashboard:**
   Create a dashboard within the UI that displays real-time statistics and historical data. This should include graphs, charts, and other visual aids to assist users in understanding the traffic patterns and signal performance.

4. **Optimization Algorithms:**
   Introduce simple optimization algorithms that analyze the collected data and suggest timing adjustments for the traffic signals to improve traffic flow and reduce wait times.

5. **Reporting and Exporting:**
   Provide functionality for generating reports summarizing the traffic conditions, signal operations, and the outcomes of any optimization processes. Include the capability to export these reports and raw data for further analysis.

**Deliverables:**

1. **Code:**
   Submit the updated Rust source files containing the implementation of the Traffic Flow Simulation, Data Collection, Analytics Dashboard, and Optimization Algorithms.

2. **UI Demonstration:**
   A gif or video of the UI showcasing the Analytics Dashboard with real-time and historical data, as well as the optimization suggestions in action.

3. **Documentation:**
   A comprehensive README file detailing your simulation engine, data collection methods, analytics tools, optimization algorithms, and how these components integrate within the overall system.

**Evaluation Criteria:**

- **Functionality:** The simulator should model complex traffic flows realistically and provide accurate, actionable data analytics.
- **Code Quality:** The code must be well-documented, maintainable, and efficient, given the increased complexity and potential for high data volumes.
- **Usability:** The analytics dashboard should present information in a clear, concise manner, allowing users to intuitively understand and interact with the data.

**Extra Credit Features:**

1. **Advanced Traffic Flow Management:**
   Develop advanced traffic management capabilities that adjust signal timings based on simulated traffic flow patterns or according to a time-of-day schedule.

2. **Machine Learning Traffic Prediction:**
   Integrate a machine learning model to predict traffic flow patterns and suggest preemptive signal timing adjustments to preemptively ease congestion.

**Hints:**
- The traffic flow simulation should include randomness but also allow for patterns that reflect typical rush hour, midday, and night-time traffic conditions.
- The analytics module will need to handle large amounts of data; efficiency in data processing and management is crucial to maintain simulator performance.

**Submission Deadline:**
[Insert deadline]

Please submit your comprehensive code repository link, along with the required deliverables, to [insert submission email or upload link].

This assignment is designed to be challenging and will require a dedicated effort to complete. You are being given extra time to ensure thoroughness in your work. This module stands to represent the culmination of the simulated SCADA-like system for traffic light control, providing a robust educational tool on the complexities of traffic management systems.

We look forward to seeing your innovative approaches to traffic analytics and optimization. Good luck, and strive for efficiency and clarity in your final submission!