using Dapr.Client;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.

builder.Services.AddControllers();
// Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();

app.UseAuthorization();


app.MapControllers();

app.MapGet("/", async() =>
{
    var client = DaprClient.CreateInvokeHttpClient("projectx-identity");
    var response = await client.GetAsync("/");
    return (await response.Content.ReadAsStringAsync());
});

app.MapGet("/order", async () =>
{
    using var client = new DaprClientBuilder().Build();
    var rnd = new Random();
    var randomNumber = rnd.Next(0, 1000000);
    await client.PublishEventAsync("pubsub", "orders", new
    {
        OrderId = randomNumber
    });
    return "Published data " + randomNumber;
});

app.MapGet("/hello", ()=>{
    return "Hello";
});
app.Run();