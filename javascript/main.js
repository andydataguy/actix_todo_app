/**
 * This function takes a list of items from the Rust backend to render into an HTML div
 *
 * @param items {Array} - list of all to do items stored in the app
 * @param processType {String} - Defines what type of process that is going to be carried out (edit/delete)
 * @param elementId {String} - the id of the HTML element where the items will be inserted
 * @param processFunction {editItem | deleteItem} - Fires the described function when the button is clicked
 */

function renderItems(items, processType, elementId, processFunction) {

  // opens up an initial div block to contain the items
  let placeholder = "<div>"

  // temporary variable to hold each item as a dictionary of ID and title after being configured for HTML
  let itemsMeta = [];

  // Loops through each stored item and extracts the title 
  for (i = 0; i < items.length; i++) {
    let title = items[i]["title"];

    // Adds the process type (edit/delete) to the extracted title. 
    // Spaces are replaced with dashes to enable use in the HTML
    // example: placeholderId = "edit-to-do-item"
    let placeholderId = processType + "-" + title.replaceAll(" ", "-");

    // Creates HTML format for the item. Adds the placeholderId to a button and 
    // inserts text for the type of process to expect
    placeholder += "<div>" 
                + title 
                + "<button " 
                + 'id="' 
                + placeholderId 
                + '">' 
                + processType 
                + '</button>' 
                + "</div>";

    // pushes the to do item into the itemsMeta array now configured for HTML
    itemsMeta.push({"id": placeholderId, "title": title});
  }

  // Once loop has finished, all items are defined with buttons and titles.
  // Closes the placeholder div block
  placeholder += "</div>"

  // inserts the placeholder into the HTML
  document.getElementById(elementId).innerHTML = placeholder;

  // Defines event listeners based on click for each ID for each individual to do item button
  for (i = 0; i < itemsMeta.length; i++) {
    document.getElementById(itemsMeta[i]["id"])
            .addEventListener("click", processFunction);
  }
}
