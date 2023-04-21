// Extracts error messages from ORY UI nodes
// (Original ORY setup provides UI 'nodes' they want used)
// Input 'e' is the raw error, and returned 'errs' is an concatenated array of each 'messages' in:
// e {
//      response: {
//          data : {
//              ui : {
//                  messages [ ... ]
//                  ...
//              }
//              messages: []
//              ...
//          }
//          ...
//      }
//  ...
// }
export function extractNestedErrorMessagesFromError(e) {
  if ('data' in e.response)
  {
    return extractNestedErrorMessagesFromData(e.response.data)
  }
  return []
}
export function extractNestedErrorMessagesFromData(data) {
  let errs = []
  if ('messages' in data.ui) {
    errs = errs.concat(data.ui.messages)
  } else if ('nodes' in data.ui) {
    // sometimes, formatted slightly differently
    for (let i = 0; i < data.ui.nodes.length; i++) {
    const node = data.ui.nodes[i]
      errs = errs.concat(node.messages)
    }
  }
  return errs
}


// Extracts csrf_token from ORY UI nodes
// (Original ORY setup provides UI 'nodes' they want used)
// Input data is the 'data' subfield of a flow data (such as from ory.getVerificationFlow())
// Returned csrf_token is nested 'value' field of attribute matching 'csrf_token' as shown below:
//    data : {
//        ui : {
//            nodes {
//                attributes {
//                   name: "csrf_token"
//                   value: <this value>
//                }
//            }
//            messages [ ... ]
//            ...
//        }
//        ...
//    }
export function extractNestedCsrfToken(data) {
  const returnedNodes = data.ui.nodes
  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].attributes.name === 'csrf_token') {
      return returnedNodes[i].attributes.value
    }
  }
  return '' // Return empty csrf token if it doesn't exist
}
