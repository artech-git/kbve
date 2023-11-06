//*       [IMPORTS]
using UnityEngine;

//?       [NPC] Class
public class NPC : MonoBehaviour
{
    public NPCData npcData; // Data object holding NPC properties
    public NPCPoolManager poolManager; // Manager handling pooling of NPCs
    public NPCAbilities abilities; // Script managing NPC abilities

    private HealthBar healthBar; // Reference to the HealthBar component
    public int currentHealth; // Current health of the NPC
    public Vector3 location; // Current location of the NPC
    public bool isFriendly; // Flag indicating if the NPC is friendly or not




    protected virtual void Start()
    {
        abilities = GetComponent<NPCAbilities>(); // Initialize abilities
        currentHealth = npcData.maxHealth; // Set current health to max
        location = transform.position; // Store initial location

        CreateHealthBar(); // Create the health bar UI
    }

    protected virtual void Update()
    {
        location = transform.position; // Update location each frame
    }

    public void ReceiveDamage(int damage)
    {
        currentHealth -= Mathf.Max(0, damage - npcData.defensePower); // Apply damage to NPC, considering defense power
        if (healthBar != null)
        {
            healthBar.SetHealth(currentHealth); // Update health bar UI
        }

        if (currentHealth <= 0)
        {
            Die(); // NPC dies if health reaches zero or below
        }
    }

    private void CreateHealthBar()
    {
        healthBar = gameObject.AddComponent<HealthBar>();
        healthBar.Initialize(npcData.maxHealth, npcData.npcName);

    }

    private void Die()
    {
        poolManager.ReturnToPool(this);
    }
}