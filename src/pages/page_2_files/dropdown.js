window.onload = setupMenu;

function setupMenu()
{
  var menus = getMenus();
  for (i = 0; i < menus.length; i++)
  {
    var menu = menus[i];
    menu.onmouseover = menuMouseOverEvent;
    menu.onmouseout = menuMouseOutEvent;
  }
}

function fixIE() {
  // The LEFT_OFFSET_FROM_CSS is the opposite of the "margin-left" value in
  // the sub-menu style in header.css.
  var LEFT_OFFSET_FROM_CSS = 7;
  
  // This is an arbitrary number that makes it line up right.
  var TOP_OFFSET = 9;
  
  var menus = getMenus();
  for (i = 0; i < menus.length; i++) {
    var menu = menus[i];
    var subMenuXOffset = (menu.offsetWidth * -1) + LEFT_OFFSET_FROM_CSS;
    var subMenuYOffset = (menu.offsetHeight - 1) - TOP_OFFSET;
    
    for (j = 0; j < menu.childNodes.length; j++) {
      var childNode = menu.childNodes[j];
      if (childNode.nodeName == "UL") {
        childNode.style.marginLeft = subMenuXOffset + "px";
        childNode.style.marginTop = subMenuYOffset + "px";
      }
    }
  }
}

function getMenus()
{
  var menus = new Array();
  var menuList = document.getElementById("linksList");
  for (i = 0; i < menuList.childNodes.length; i++)
  {
    var menu = menuList.childNodes[i];
    if ((menu.nodeName == "LI") && (menu.className.match(/menu/)))
    {
      menus[menus.length] = menu;
    }
  }
  
  return menus;
}

function menuMouseOverEvent()
{
   this.className = "over";
}


function menuMouseOutEvent()
{
   this.className = "menu";
}