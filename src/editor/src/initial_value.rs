pub fn initial_value() -> &'static str {
    let html: &'static str = r#"

    <h1 >heading 1</h1>

    <h2></h2>
    <h2 empty="true"></h2>

    <div  >Div obj</div>
    <h1 horizontal="true" >Horizontal objects</h1>

    <h1 horizontal="true">Horizontal object 2</h1>

    <h2>heading 2</h2>
    <h3 >heading 3</h3>
    <h4>heading 4</h4>
    <h5>heading 5</h5>
    <h6>heading 6</h6>
    <p>Paragraphe is here
    <br>
    <p>     sub paaragaphe is here.</p>
    </p>


<ol>
  <lh>ol Header</lh>
  <li>Item One</li>
  <li>Item Two</li>
  <li>Item Thre</li>
</ol>


    <br>
    <table>
  <tr>
    <th>Firstname</th>
    <th>Lastname</th>
  </tr>
  <tr>
    <td>Peter</td>
    <td>Griffin</td>
  </tr>
  <tr>
    <td>Lois</td>
    <td>Griffin</td>
  </tr>
</table>




<h1> at the buttom </h1>
<h1> at the buttom 2 </h1>




    "#;
    return html;
    // return <()>::try_from(html).unwrap();
}
