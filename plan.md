# Program Structure Plan

1. **Create the main application struct:**
   - Define a new struct to represent the state of your journal application. This struct will hold various components, including the current calendar view and a vector to store the journal entries.

2. **Implement the calendar system (for daily journaling):**
   - Use Ratatui's `calendar` widget to create a calendar view.
   - Utilize an event loop to listen for user input on the calendar and update the application state accordingly.

3. **Timestamping entries:**
   - Add a timestamp field to your journal entry struct, automatically updating when an entry is created.

4. **Tag entries:**
   - Implement a tagging system by adding a vector of tags to your journal entry struct.
   - Create a UI component for selecting and adding tags.

5. **Past entry viewer:**
   - Create a new screen or view for displaying past journal entries.
   - Use Ratatui's `text_buffer` widget to display the content of each entry.
   - Implement functionality to filter entries by date and tag.

6. **Sort by day/week/month/year:**
   - Modify your calendar system and past entry viewer to support sorting entries by desired time periods.
   
7. **User interface design:**
   - Design a simple and intuitive user interface using Ratatui's widget library.
   - Ensure all components are easily accessible and well-organized.
