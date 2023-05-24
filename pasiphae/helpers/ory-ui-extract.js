// Extracts error messages from ORY UI nodes
// (Original ORY setup provides UI 'nodes' they want used)
// }
export function extractNestedErrorMessagesFromError(e) {
  let errs = []
  if ('data' in e.response) {
    if ('error' in e.response.data) {
      // Non-UI (but still Ory-recognized) error returned, will have 'reason' field
      errs.push({ id: 0, type: 'error', text: e.response.data.error.reason })
    } else if ('ui' in e.response.data) {
      return extractNestedErrorMessagesFromUiData(e.response.data)
    }
  }
  return errs
}
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
export function extractNestedErrorMessagesFromUiData(data) {
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

// Extracts providers from ORY UI nodes
// labeled with 'provider' attribute
const preferred_order = ['github', 'discord', 'google', 'apple', 'microsoft', 'gitlab']
export function extractOidcProviders(data) {
  let providers = []
  const returnedNodes = data.ui.nodes
  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].group === 'oidc' && returnedNodes[i].attributes.name === 'provider') {
      providers.push(returnedNodes[i].attributes.value)
    }
  }
  return providers.sort((a, b) => preferred_order.indexOf(a) - preferred_order.indexOf(b))
}
// labeled with 'link' attribute
export function extractOidcLinkProviders(data) {
  let providers = []
  const returnedNodes = data.ui.nodes
  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].group === 'oidc' && returnedNodes[i].attributes.name === 'link') {
      providers.push(returnedNodes[i].attributes.value)
    }
  }
  return providers.sort((a, b) => preferred_order.indexOf(a) - preferred_order.indexOf(b))
}
// labeled with 'unlink' attribute
export function extractOidcUnlinkProviders(data) {
  let providers = []
  const returnedNodes = data.ui.nodes
  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].group === 'oidc' && returnedNodes[i].attributes.name === 'unlink') {
      providers.push(returnedNodes[i].attributes.value)
    }
  }
  return providers.sort((a, b) => preferred_order.indexOf(a) - preferred_order.indexOf(b))
}
